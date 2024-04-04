use std::sync::{Arc, Mutex};
use std::time::{UNIX_EPOCH, SystemTime};
use datatypes::ImportantFunctions as _;
use fast_protocol::datatypes::packets::KeepAlive;
use openssl::sha::Sha1;
use rand::RngCore as _;
use tokio::net::tcp::WriteHalf;
use tokio::sync::{mpsc, oneshot};
use tokio::io::{AsyncWrite, BufReader};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{Instant, Duration};

use fast_protocol::datatypes::{packets::{State, ServerboundPackets, LegacyPongPacket, PongPacket, StatusResponsePacket, LoginSuccess}, datatype_definition::important_enums::HandshakeNextState};
use binary_utils::DataWriter;

use crate::{server::{ServerMessage, server_settings::ServerSettings}, player::{Player, PlayerMessages}};

const _UPDATE_RATE: f32 = 100.0;

pub struct ConnectionHandler;
#[derive(Debug)]
pub enum ConnectionHandlerError {
    ThreadMovement(String),
    PacketSent(String),
    Shutdown(String),
    StartSequence(String),
    PacketReading(binary_utils::Error),
    ReponseError,
    ChannelError,
    KickingPlayer,
}
impl From<binary_utils::Error> for ConnectionHandlerError {
    fn from(value: binary_utils::Error) -> Self {
        Self::PacketReading(value)
    }
}
#[derive(Debug)]
pub struct Error {
    pub error_type: ConnectionHandlerError,
    pub player: Option<Arc<Mutex<fast_protocol::datatypes::json_datastructures::Player>>>,
}
type ErrorResult = std::result::Result<(), ConnectionHandlerError>;
async fn write(packet: &impl DataWriter, writer: &mut (impl AsyncWrite + Unpin + Send)) -> ErrorResult {
    match packet.write(writer).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ConnectionHandlerError::PacketSent(format!("{}:{}", file!(), line!()))),
    }
}
fn minecraft_sha1_hexdigest(input: Sha1) -> String {
    let result = input.finish();

    // Convert the SHA-1 hash to a hexadecimal string
    let mut hex_digest = std::string::String::new();
    for byte in &result {
        hex_digest += &format!("{:02x}", byte);
    }

    // Handle Minecraft's non-standard formatting
    if result[0] & 0x80 != 0 {
        // If the first byte is negative, prepend a minus sign
        format!("-{}", hex_digest)
    } else {
        hex_digest
    }
}
const KICK_TIME: Duration = Duration::from_secs(30);
impl ConnectionHandler {
    // TODO: use this or delete it
    async fn _handle_incomming_messages(mut _stream: &TcpStream, mut receiver: mpsc::Receiver<PlayerMessages>) {
        while let Some(message) = receiver.recv().await {
            match message {
            }
        }
    }
    async fn send_keep_alive<'a>(
        writer: &mut WriteHalf<'a>,
        last_keep_alive_send: &mut Instant,
        keep_alive_answered_at: &Instant,
        await_keep_alive_answer: &mut bool,
        player: &Option<Player>
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        let now = Instant::now();
        if !now.duration_since(*last_keep_alive_send).lt(&KICK_TIME) {
            if *await_keep_alive_answer || (*last_keep_alive_send).saturating_duration_since(*keep_alive_answered_at).ge(&KICK_TIME) {
                return Self::disconnect_player(player.clone());
            } else {
                let kap = KeepAlive::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
                if let Err(e) = kap.write(writer).await {
                    return Err((ConnectionHandlerError::PacketSent(format!("{:?}", e)), player.clone()));
                } else {
                    *last_keep_alive_send = now;
                    *await_keep_alive_answer = true;
                }
            }
        }
        Ok(())
    }
    fn disconnect_player(player: Option<Player>) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        return Err((ConnectionHandlerError::KickingPlayer, player));
    }
    pub(crate) async fn run(mut stream: TcpStream,
        sender: mpsc::Sender<ServerMessage>,
        settings: ServerSettings,
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)>{
        println!("Connected");
        // let server = &server;
        let mut player: Option<Player> = None;
        let mut state = State::Handshake;
        // let stream_clone = stream.try_clone().unwrap();
        // let mut buf_reader = BufReader::new(stream);
        // let buf_writer = BufWriter::new(stream);
        let (player_sender, mut _player_receiver) = mpsc::channel::<PlayerMessages>(20);
        // TODO use player_receiver to receive packages
        // tokio::spawn(async move {while let Some(message) = player_receiver.recv().await {match message {}}});
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        let mut verify_token = [0;4];
        let mut last_keep_alive_send = Instant::now();
        let mut keep_alive_answered_at = Instant::now();
        // let mut keep_alive_answered_after = keep_alive_answered_at.duration_since(last_keep_alive_send).as_millis();
        let mut await_keep_alive_answer = false;
        rand::thread_rng().fill_bytes(&mut verify_token);
        let mut last_checked = Instant::now();
        let update_time = tokio::time::Duration::from_secs(1).div_f64(100.0);
        loop {
            Self::send_keep_alive(&mut writer, &mut last_keep_alive_send, &keep_alive_answered_at, &mut await_keep_alive_answer, &player).await?;
            let packet = ServerboundPackets::read(&mut reader, state).await;
            if let Err(err) = packet {
                return Err((ConnectionHandlerError::from(err), player.clone()));
            }
            let packet = packet.unwrap();
            match packet {
                ServerboundPackets::None => (),
                ServerboundPackets::LegacyPing => {
                    println!("Legacy Ping");
                    let (player_sender, player_rec) = oneshot::channel();
                    match sender.send(ServerMessage::GetPlayerAmount(player_sender)).await {
                        Ok(()) => (),
                        Err(_) => return Err((ConnectionHandlerError::ReponseError, player)),
                    }
                    let player_amount = match player_rec.await {
                        Ok(v) => v,
                        Err(_) => return Err((ConnectionHandlerError::ReponseError, player)),
                    };
                    let packet = LegacyPongPacket::new(settings.version.to_string(), settings.motd.clone(), player_amount, settings.max_players);
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
                    match write(&pong, &mut writer).await {
                        Ok(_) => Ok(()),
                        Err(err) => Err((err, player.clone())),
                    }?
                },
                ServerboundPackets::StatusRequest(_req) => {
                    let (player_sender, player_rec) = oneshot::channel();
                    if let Err(_) = sender.send(ServerMessage::GetPlayers(player_sender)).await {
                        return Err((ConnectionHandlerError::ChannelError, player));
                    }
                    let players = match player_rec.await {
                        Ok(v) => v,
                        Err(_) => return Err((ConnectionHandlerError::ReponseError, player)),
                    };
                    let res = StatusResponsePacket::new(
                        settings.version.to_string(),
                        settings.protocol_version,
                        settings.max_players,
                        players.len() as u16,
                        players.iter().map(|p| fast_protocol::datatypes::json_datastructures::Player::new(p.username.clone(), p.uuid.to_string())).collect(),
                        settings.motd.clone(),
                    );
                    // write(&res, &mut stream);
                    if let Err(err) = res.write(&mut writer).await {
                        return Err((ConnectionHandlerError::from(err), player));
                    }
                },
                ServerboundPackets::LoginStart(req) => {
                    println!("Username: {}", req.name.get_value());
                    player = Some(Player { uuid: req.uuid.get_value(), username: req.name.get_value(), sender: player_sender.clone() });
                    if settings.offline_mode {
                        let res = LoginSuccess::new(req.uuid, req.name);
                        match res.write(&mut writer).await {
                            Ok(_) => (),
                            Err(err) => return Err((ConnectionHandlerError::from(err), player)),
                        }
                    } else {
                        // todo request rsa key from server via sender.send
                        // let res = LoginEncryptionRequest::new(server.rsa_key.clone(), verify_token);
                        // res.write(&mut writer).await?;
                    }
                }
                ServerboundPackets::LoginEncryptionResponse(res) => {
                    println!("Reading this:");
                    let shared_secret = res.shared_secret.get_value();
                    let mut _new_shared_secret = vec![0;shared_secret.len()+500];
                    println!("{:?}", shared_secret);
                    println!("Beeing here (1) {}", shared_secret.len());
                    // TODO decript via requested provate key with sender.send
                    /*
                    server.rsa_key.clone().private_decrypt(
                        &shared_secret,
                        &mut new_shared_secret,
                        Padding::PKCS1);
                    */
                    println!("Beeing here (2)");
                    let mut sha1 = openssl::sha::Sha1::new();
                    sha1.update(&shared_secret);
                    // TODO use requested rsa key
                    // sha1.update(&server.rsa_key.clone().public_key_to_der().unwrap());
                    let hash = minecraft_sha1_hexdigest(sha1);
                    let username = player.clone().unwrap().username.clone();
                    let url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hash}");
                    let _response = match fetch_url(&url).await {
                        Ok(v) => Ok(v),
                        Err(_) => Err((ConnectionHandlerError::PacketSent("".to_string()), player.clone())),
                    }?;
                    println!("Being here!");
                },
                ServerboundPackets::LoginAcknowledged => {
                    match sender.send(ServerMessage::AddPlayer(player.clone())).await {
                        Ok(()) => (),
                        Err(_) => return Err((ConnectionHandlerError::ReponseError, player)),
                    }
                }
                ServerboundPackets::Pong => unreachable!(),
                ServerboundPackets::KeepAlive(_keep_alive) => {
                    keep_alive_answered_at = Instant::now();
                    await_keep_alive_answer = false;
                },
                ServerboundPackets::ClientInformation(_client_information) => todo!(),
                ServerboundPackets::AckFinishConfiguration => todo!(),
                ServerboundPackets::ResoucePackResponse(_res) => todo!(),
                ServerboundPackets::ConfigurationPluginMessage(_message) => todo!(),
            }
            let now = Instant::now();
            let sleep_time = now.duration_since(last_checked).saturating_sub(update_time);
            last_checked = now;
            tokio::time::sleep(sleep_time).await;
        }
        Ok(())
    }
}
async fn fetch_url(url: &str) -> reqwest::Result<String> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
