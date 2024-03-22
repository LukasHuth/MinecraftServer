use std::{sync::{Arc, Mutex}, time::{Instant, Duration}, ops::Sub, thread::{self}};

use tokio::{net::TcpStream, io::{AsyncWrite, BufReader, BufWriter}};
use tokio::io::AsyncWriteExt;

use fast_protocol::{utils::DataWriter, datatypes::{datatype_definition::important_enums::HandshakeNextState, packets::{PongPacket, StatusResponsePacket}} };

use fast_protocol::datatypes::datatype_definition::ImportantFunctions;

use fast_protocol::datatypes::packets::{State, ServerboundPackets, LegacyPongPacket};

use crate::{server::Server, player::Player};

const UPDATE_RATE: f32 = 100.0;

pub struct ConnectionHandler;
#[derive(Debug)]
pub enum ConnectionHandlerError {
    ThreadMovement(String),
    PacketSent(String),
    Shutdown(String),
    StartSequence(String),
    PacketReading(fast_protocol::errors::Error)
}
impl From<fast_protocol::errors::Error> for ConnectionHandlerError {
    fn from(value: fast_protocol::errors::Error) -> Self {
        Self::PacketReading(value)
    }
}
#[derive(Debug)]
pub struct Error {
    pub error_type: ConnectionHandlerError,
    pub player: Option<Arc<Mutex<fast_protocol::datatypes::json_datastructures::Player>>>,
}
type ErrorResult = std::result::Result<(), ConnectionHandlerError>;
macro_rules! create_error {
    (ThreadMovement, $player:expr) => {
            ConnectionHandlerError::ThreadMovement(format!("Error at: {}:{}", file!(), line!()))
    };
    (StartSequence, $player:expr) => {
            ConnectionHandlerError::StartSequence(format!("Error at: {}:{}", file!(), line!()))
    };
    (PacketSent, $player:expr) => {
            ConnectionHandlerError::PacketSent(format!("Error at: {}:{}", file!(), line!()))
    };
    (Shutdown, $player:expr) => {
        ConnectionHandlerError::Shutdown(format!("Error at: {}:{}", file!(), line!()))
    };
    (PacketReading, $err:expr, $player:expr) => {
            ConnectionHandlerError::PacketReading($err),
    };
}
macro_rules! unlock {
    ($server:expr) => {
        $server
    };
}
async fn write(packet: &impl DataWriter, writer: &mut (impl AsyncWrite + Unpin + Send)) -> ErrorResult {
    match packet.write(writer).await {
        Ok(_) => Ok(()),
        Err(_) => Err(create_error!(PacketSent, player)),
    }
}
impl ConnectionHandler {
    pub async fn run(mut stream: TcpStream, server: Arc<Server>) -> Result<(), ConnectionHandlerError>{
        println!("Connected");
        let server = &server;
        let mut player: Option<Player> = None;
        let mut state = State::Handshake;
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        // let update_time_ms = f32::floor(1000.0 / UPDATE_RATE) as u64;
        // let mut last_update = Instant::now().sub(Duration::from_millis(update_time_ms));
        loop {
            // let delta = Instant::now().duration_since(last_update).as_millis() as u64;
            /*
            if delta < update_time_ms {
                thread::sleep(Duration::from_millis(update_time_ms - delta));
            }
            last_update -= Duration::from_millis(update_time_ms);
            */
            if let Some(mut player) = player.clone() {
                while let Some(packet) = player.queue.pop_front() {
                    packet.write(&mut writer).await?;
                }
            }
            let packet = ServerboundPackets::read(&mut reader, state).await;
            if let Err(err) = packet {
                return Err(ConnectionHandlerError::from(err));
            }
            let packet = packet.unwrap();
            match packet {
                ServerboundPackets::None => (),
                ServerboundPackets::LegacyPing => {
                    println!("Legacy Ping");
                    let settings = &server.settings;
                    let packet = LegacyPongPacket::new(settings.version.to_string(), settings.motd.clone(), server.players.len() as u16, settings.max_players);
                    match packet.write(&mut writer).await {
                        Ok(_) => (),
                        Err(_) => break,
                    }
                    if let Err(_) = stream.shutdown().await {
                        break;
                    }
                    return Ok(());
                }
                ServerboundPackets::Handshake(handshake) => {
                    println!("next_state: {:?}", handshake.next_state);
                    match handshake.next_state.get_value() {
                        HandshakeNextState::Login => state = State::Login,
                        HandshakeNextState::Status => state = State::Status,
                    }
                },
                ServerboundPackets::PingRequest(ping) => {
                    let pong = PongPacket::new(ping.id.get_value());
                    write(&pong, &mut writer).await?;
                },
                ServerboundPackets::StatusRequest(_req) => {
                    let server = unlock!(server);
                    let settings = &server.settings;
                    let res = StatusResponsePacket::new(
                        settings.version.to_string(),
                        settings.protocol_version,
                        settings.max_players,
                        server.players.len() as u16,
                        vec![],
                        settings.motd.clone(),
                    );
                    // write(&res, &mut stream);
                    if let Err(err) = res.write(&mut writer).await {
                        return Err(ConnectionHandlerError::from(err));
                    }
                },
            }
        }
        Ok(())
    }
}
