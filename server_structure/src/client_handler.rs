use std::{net::TcpStream, sync::{Mutex, Arc}, io::Write};

use packet_api::datatypes::packet_implementation::{PacketAnalyzer, Packet, packets::{LegacyPong, StatusResponse, PongResponse, Packet as _, LoginSuccess}, PacketAnalyserError};

use crate::{server::{Server, ServerEvent}, player::Player};
pub struct ClientHandler;

pub enum ClientHandlerError<'a> {
    ThreadMovement(&'a str),
    PacketSent(&'a str),
    Shutdown(&'a str),
    PacketReading(PacketAnalyserError),
}
pub struct Error<'a> {
    pub error_type: ClientHandlerError<'a>,
    pub player: Option<packet_api::datatypes::datastructs::player::Player>,
}
impl<'a> Error<'a> {
    pub fn new(player: Option<packet_api::datatypes::datastructs::player::Player>, error_type: ClientHandlerError<'a>) -> Self {
        Self {player, error_type }
    }
}
macro_rules! streamwrite {
    ($stream:expr, $data:expr, $client_connection:expr) => {
        match $stream.write_all($data) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new($client_connection.get_player(), ClientHandlerError::PacketSent("Packet could not be sent"))),
        }?;
    };
}
impl<'a> ClientHandler {
    pub async fn run(mut stream: TcpStream, server: &Arc<Mutex<Server>>) -> Result<(), Error<'a>>{
        // let mut client_clone = client.try_clone().expect("Couldnt create Connection clone");
        let mut client_clone = stream.try_clone().unwrap();
        let mut client_connection = PacketAnalyzer::new(&mut client_clone);
        loop {
            if let Some(mut player) = client_connection.player.clone() {
                while let Some(packet) = player.queue.pop_front() {
                    let packet = match packet.lock() {
                        Ok(guard) => Ok(guard),
                        Err(_) => Err(Error::new(client_connection.get_player(), ClientHandlerError::ThreadMovement(""))),
                    }?;
                    streamwrite!(stream, &packet.to_bytes(), client_connection);
                }
            }
            let packet = client_connection.next_packet();
            let packet = match packet {
                Ok(packet) => Ok(packet),
                Err(err) => Err(Error::new(client_connection.get_player(), ClientHandlerError::PacketReading(err)))
            }?;
            match packet {
                Packet::LegacyPing(_) => {
                    let server = server.lock().unwrap();
                    let player_count = server.players.len();
                    let settings = &server.server_settings;
                    let pong = LegacyPong::new(settings.version, settings.motd, player_count as u16, settings.max_players);
                    let data = pong.to_bytes();
                    streamwrite!(stream, &data, client_connection);
                    match stream.shutdown(std::net::Shutdown::Both) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error::new(client_connection.get_player(), ClientHandlerError::Shutdown("Connection closing did not work")))
                    }?;
                    stream.shutdown(std::net::Shutdown::Both).expect("Could not close the connection");
                    break;
                },
                Packet::StatusRequest => {
                    let server = server.lock().expect("Could not reserve server info");
                    let settings = &server.server_settings;
                    let response = StatusResponse::new(
                        settings.version.to_string(),
                        settings.protocol_version,
                        settings.max_players,
                        server.get_players().iter().map(Player::into_packet_player).collect(),
                        settings.motd.to_string(),
                    );
                    let data = response.to_bytes();
                    streamwrite!(stream, &data, client_connection);
                },
                Packet::LoginStart(login_start) => {
                    let server = server.lock().expect("Could not reserver server info");
                    let settings = server.server_settings.clone();
                    if settings.offline_mode {
                        client_connection.skip_authentication();
                        let login_success = LoginSuccess::new(login_start.uuid.clone(), login_start.name.clone());
                        let data = login_success.to_bytes();
                        streamwrite!(stream,&data, client_connection);
                        client_connection.create_player(login_start.name, login_start.uuid);
                        continue;
                    }
                },
                Packet::LoginAck => {
                    let mut server = server.lock().expect("Could not get a mutable server instance");
                    server.trigger(ServerEvent::PlayerJoined(Player::from(client_connection.player.clone().unwrap())));
                },
                Packet::Ping(ping) => {
                    let pong = PongResponse::new(ping);
                    let data = pong.to_bytes();
                    stream.write_all(&data).expect("Could not send the Pong Response Packet");
                },
                Packet::None => (),
            }
        }
        Ok(())
    }
}
