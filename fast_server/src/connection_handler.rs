use datatypes::ImportantFunctions as _;
use fast_protocol::datatypes::datatype_definition::important_enums::HandshakeNextState;
use fast_protocol::datatypes::packets::configuration::clientbound::{AddResourcePack, ConfigurationClientboundPluginMessage, ConfigurationDisconnect, FeatureFlags, FinishConfiguration, KeepAlive, RegistryData, RemoveResoucePack};
use fast_protocol::datatypes::packets::handshake::LegacyPongPacket;
use fast_protocol::datatypes::packets::login::LoginSuccess;
use fast_protocol::datatypes::packets::status::{PongPacket, StatusResponsePacket};
use fast_protocol::datatypes::packets::{ClientboundPackets, ServerboundPackets, State};
use openssl::sha::Sha1;
use rand::RngCore as _;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncWrite, BufReader};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Duration, Instant};

// use fast_protocol::datatypes::packets::{KeepAlive, ConfigurationDisconnect, ClientboundPackets, ConfigurationClientboundPluginMessage, RegistryData, RemoveResoucePack, AddResourcePack, FeatureFlags, FinishConfiguration};
// use fast_protocol::datatypes::{packets::{State, ServerboundPackets, LegacyPongPacket, PongPacket, StatusResponsePacket, LoginSuccess}, datatype_definition::important_enums::HandshakeNextState};
use binary_utils::DataWriter;

use crate::server::ConfigData;
use crate::{
    player::{Player, PlayerMessages},
    server::{server_settings::ServerSettings, ServerMessage},
};

const _UPDATE_RATE: f32 = 100.0;

/// Struct used to handle connections of the players
pub struct ConnectionHandler;
/// Error enum of the different errors that can occur
#[derive(Debug)]
pub enum ConnectionHandlerError {
    /// This error occurs, if an object could not be moved between two threads
    ThreadMovement(String),
    /// This error occurs, if the server was not able to send an packet to the player
    PacketSent(String),
    /// This error is and error used when the server closes the connection
    Shutdown(String),
    /// This error occurs, if the start sequence is not being used
    StartSequence(String),
    /// This error occurs, if an error occurs while reading a packet
    PacketReading(binary_utils::Error),
    /// This error occurs, if a response is not how it es expected
    ReponseError,
    /// This error occurs, if a channel produces an error
    ChannelError,
    /// This error is being used, if the server kicks a player
    KickingPlayer,
    /// This error is being used, if the thread wants to close the `TcpStream` beacause of
    /// inactivity
    CloseConnection,
}
impl From<binary_utils::Error> for ConnectionHandlerError {
    fn from(value: binary_utils::Error) -> Self {
        Self::PacketReading(value)
    }
}
/// An custom error type that keeps track of the error type and the player that the thread was
/// handling
#[derive(Debug)]
pub struct Error {
    /// Type of the error
    pub error_type: ConnectionHandlerError,
    /// instance of the player that the thread handled if one exists
    pub player: Option<Arc<Mutex<fast_protocol::datatypes::json_datastructures::Player>>>,
}
type ErrorResult = std::result::Result<(), ConnectionHandlerError>;
async fn write(
    packet: &impl DataWriter,
    writer: &mut (impl AsyncWrite + Unpin + Send),
) -> ErrorResult {
    match packet.write(writer).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ConnectionHandlerError::PacketSent(format!(
            "{}:{}",
            file!(),
            line!()
        ))),
    }
}
fn minecraft_sha1_hexdigest(input: Sha1) -> String {
    let result = input.finish();
    let mut hex_digest = std::string::String::new();
    for byte in &result {
        hex_digest += &format!("{:02x}", byte);
    }
    if result[0] & 0x80 != 0 {
        format!("-{}", hex_digest)
    } else {
        hex_digest
    }
}
const KICK_TIME: Duration = Duration::from_secs(15);
impl ConnectionHandler {
    async fn handle_incomming_messages<'a>(
        _writer: &mut WriteHalf<'a>,
        receiver: &mut mpsc::Receiver<PlayerMessages>,
        _packet_queue: &mut VecDeque<ClientboundPackets>,
    ) {
        while let Ok(message) = receiver.try_recv() {
            match message {}
        }
    }
    /// function to handle the keep alive
    ///
    /// # Note
    ///
    /// This function checks if it has to send/receive a packet. it can be run every iteration
    async fn send_keep_alive<'a>(
        state: &State,
        writer: &mut WriteHalf<'a>,
        last_keep_alive_send: &mut Instant,
        keep_alive_answered_at: &Instant,
        await_keep_alive_answer: &mut bool,
        player: &Option<Player>,
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        let now = Instant::now();
        if !now.duration_since(*last_keep_alive_send).lt(&KICK_TIME) {
            match state {
                State::Login | State::Status | State::Handshake => {
                    return Err((ConnectionHandlerError::CloseConnection, None))
                }
                State::Configuration => {
                    if *await_keep_alive_answer
                        || (*last_keep_alive_send)
                            .saturating_duration_since(*keep_alive_answered_at)
                            .ge(&KICK_TIME)
                    {
                        return Self::disconnect_player(player.clone());
                    } else {
                        let kap = KeepAlive::new(
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs() as i64,
                        );
                        if let Err(e) = kap.write(writer).await {
                            return Err((
                                ConnectionHandlerError::PacketSent(format!("{:?}", e)),
                                player.clone(),
                            ));
                        } else {
                            *last_keep_alive_send = now;
                            *await_keep_alive_answer = true;
                        }
                    }
                }
                State::Playing => {
                    todo!()
                }
            }
        }
        Ok(())
    }
    /// function to disconnect the player
    ///
    /// # Arguments
    /// `player` - optional player instance
    fn disconnect_player(
        player: Option<Player>,
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        return Err((ConnectionHandlerError::KickingPlayer, player));
    }
    /// function to handle/receive packets
    ///
    /// # Arguments
    /// `reader` - A `BufReader<ReadHalf>` to read data from the stream
    /// `writer` - A `WriteHalf` to write data into the stream
    /// `sender` - A `Sender<ServerMessage` to be able to communicate with the server
    /// `state` - The current state of the connection
    /// `player` - An optional instance of the player of the connection
    /// `settings` - A reference to the `ServerSettings`
    /// `keep_alive_answered_at` - A mutable reference to an `Instant` that records the last time,
    /// that an `KeepAlive` was answered at
    /// `await_keep_alive_answer` - A mutable reference to a bool whether an keep alive is awaited
    /// or not
    /// `player_sender` - A reference to the player sender instance to read from it
    /// `packet_queue` - A mutable reference to a `VecDeque<ClientboundPackets>` to send packets,
    /// that are queued to be send to the player
    async fn receive_packets<'a>(
        reader: &mut BufReader<ReadHalf<'a>>,
        writer: &mut WriteHalf<'a>,
        sender: &mpsc::Sender<ServerMessage>,
        state: &mut State,
        player: &mut Option<Player>,
        settings: &ServerSettings,
        keep_alive_answered_at: &mut Instant,
        await_keep_alive_answer: &mut bool,
        player_sender: &Sender<PlayerMessages>,
        packet_queue: &mut VecDeque<ClientboundPackets>,
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        let packet = ServerboundPackets::read(reader, state).await;
        if let Err(err) = packet {
            return Err((ConnectionHandlerError::from(err), player.clone()));
        }
        let packet = packet.unwrap();
        match packet {
            ServerboundPackets::None => {}
            ServerboundPackets::LegacyPing => {
                println!("Legacy Ping");
                let (player_sender, player_rec) = oneshot::channel();
                match sender
                    .send(ServerMessage::GetPlayerAmount(player_sender))
                    .await
                {
                    Ok(()) => (),
                    Err(_) => return Err((ConnectionHandlerError::ReponseError, player.clone())),
                }
                let player_amount = match player_rec.await {
                    Ok(v) => v,
                    Err(_) => return Err((ConnectionHandlerError::ReponseError, player.clone())),
                };
                let packet = LegacyPongPacket::new(
                    settings.version.to_string(),
                    settings.motd.clone(),
                    player_amount,
                    settings.max_players,
                );
                match packet.write(writer).await {
                    Ok(_) => (),
                    Err(_) => return Err((ConnectionHandlerError::CloseConnection, None)),
                }
                return Err((ConnectionHandlerError::CloseConnection, None));
            }
            ServerboundPackets::Handshake(handshake) => {
                println!("next_state: {:?}", handshake.next_state);
                match handshake.next_state.get_value() {
                    HandshakeNextState::Login => *state = State::Login,
                    HandshakeNextState::Status => *state = State::Status,
                }
            }
            ServerboundPackets::PingRequest(ping) => {
                let pong = PongPacket::new(ping.id.get_value());
                match write(&pong, writer).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err((err, player.clone())),
                }?
            }
            ServerboundPackets::StatusRequest(_req) => {
                let (player_sender, player_rec) = oneshot::channel();
                if let Err(_) = sender.send(ServerMessage::GetPlayers(player_sender)).await {
                    return Err((ConnectionHandlerError::ChannelError, player.clone()));
                }
                let players = match player_rec.await {
                    Ok(v) => v,
                    Err(_) => return Err((ConnectionHandlerError::ReponseError, player.clone())),
                };
                let res = StatusResponsePacket::new(
                    settings.version.to_string(),
                    settings.protocol_version,
                    settings.max_players,
                    players.len() as u16,
                    players
                        .iter()
                        .map(|p| {
                            fast_protocol::datatypes::json_datastructures::Player::new(
                                p.username.clone(),
                                p.uuid.to_string(),
                            )
                        })
                        .collect(),
                    settings.motd.clone(),
                );
                if let Err(err) = res.write(writer).await {
                    return Err((ConnectionHandlerError::from(err), None));
                }
            }
            ServerboundPackets::LoginStart(req) => {
                println!("Username: {}", req.name.get_value());
                *player = Some(Player {
                    uuid: req.uuid.get_value(),
                    username: req.name.get_value(),
                    sender: player_sender.clone(),
                    information: None,
                });
                if settings.offline_mode {
                    let res = LoginSuccess::new(req.uuid, req.name);
                    match res.write(writer).await {
                        Ok(_) => (),
                        Err(err) => {
                            return Err((ConnectionHandlerError::from(err), player.clone()))
                        }
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
                let mut _new_shared_secret = vec![0; shared_secret.len() + 500];
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
                    Err(_) => Err((
                        ConnectionHandlerError::PacketSent("".to_string()),
                        player.clone(),
                    )),
                }?;
                println!("Being here!");
            }
            ServerboundPackets::LoginAcknowledged => {
                match sender.send(ServerMessage::AddPlayer(player.clone())).await {
                    Ok(()) => {
                        *state = State::Configuration;
                    }
                    Err(_) => return Err((ConnectionHandlerError::ReponseError, player.clone())),
                }
            }
            ServerboundPackets::Pong => unreachable!(),
            ServerboundPackets::KeepAlive(_keep_alive) => {
                *keep_alive_answered_at = Instant::now();
                *await_keep_alive_answer = false;
            }
            ServerboundPackets::ClientInformation(client_information) => {
                if let Some(player) = player {
                    println!("Received Client Information");
                    use datatypes::ImportantFunctions;
                    let locale = client_information.locale.get_value();
                    let view_distance = client_information.view_distance.get_value() as u8;
                    let chat_mode = client_information.chat_mode.get_value();
                    let chat_colors = client_information.chat_colors.get_value();
                    let displayed_skin_parts = client_information.displayed_skin_parts.get_value();
                    let main_hand = client_information.main_hand.get_value();
                    let text_filtering = client_information.text_filtering.get_value();
                    let in_server_listing = client_information.allow_server_listing.get_value();
                    player.information = Some(crate::player::PlayerInformation {
                        locale,
                        view_distance,
                        chat_mode,
                        chat_colors,
                        displayed_skin_parts,
                        main_hand,
                        text_filtering,
                        in_server_listing,
                    });
                    println!("Finished with this");
                }
                let (config_packets_sender, config_packets_receiver) =
                    oneshot::channel::<Vec<ConfigData>>();
                if let Err(_) = sender
                    .send(ServerMessage::GetConfigData(config_packets_sender))
                    .await
                {
                    return Err((ConnectionHandlerError::ChannelError, player.clone()));
                }
                let response = match config_packets_receiver.await {
                    Ok(v) => v,
                    Err(_) => return Err((ConnectionHandlerError::ChannelError, player.clone())),
                };
                let response = response.iter().map(|e| match e {
                    ConfigData::PluginMessage(channel, data) => {
                        ClientboundPackets::ConfigurationPluginMessage(
                            ConfigurationClientboundPluginMessage::new(
                                datatypes::Identifier::new(channel.clone()),
                                data.clone(),
                            ),
                        )
                    }
                    ConfigData::RegistryData(nbt) => {
                        ClientboundPackets::RegistryData(RegistryData::new(nbt.clone()))
                    }
                    ConfigData::RemoveResourcePack(uuid) => {
                        ClientboundPackets::RemoveResourcePack(RemoveResoucePack::new(uuid.clone()))
                    }
                    ConfigData::AddResourcePack(uuid, url, hash, forced, prompt_message) => {
                        ClientboundPackets::AddResourcePack(AddResourcePack::new(
                            *uuid,
                            url.clone(),
                            hash.clone(),
                            *forced,
                            prompt_message.clone(),
                        ))
                    }
                    ConfigData::FeatureFlags(flags) => {
                        ClientboundPackets::FeatureFlags(FeatureFlags::new(
                            flags
                                .iter()
                                .map(|s| datatypes::Identifier::new(s.clone()))
                                .collect(),
                        ))
                    }
                });
                packet_queue.extend(response);
                packet_queue.push_back(ClientboundPackets::FinishConfiguration(
                    FinishConfiguration::new(),
                ));
                println!("queued packets: {}", packet_queue.len());
            }
            ServerboundPackets::AckFinishConfiguration => {
                println!("Switching to playing");
                *state = State::Playing;
            }
            ServerboundPackets::ResoucePackResponse(res) => {
                use datatypes::ImportantFunctions;
                use fast_protocol::datatypes::packets::configuration::serverbound::ResourcePackResponseEnum as RPRE;
                let uuid = res.uuid;
                let response = res.result.get_value();
                let applied: bool = match response {
                    RPRE::Declined
                    | RPRE::Discarded
                    | RPRE::InvalidUrl
                    | RPRE::FailedToReload
                    | RPRE::FailedToDownload => false,
                    RPRE::Accepted | RPRE::Downloaded | RPRE::SuccessfullyDownloaded => true,
                };
                let (answer_sender, answer_receiver) = tokio::sync::oneshot::channel::<bool>();
                let _ = sender
                    .send(ServerMessage::IsResourcePackImportant(uuid, answer_sender))
                    .await;
                let is_important = answer_receiver.await.unwrap_or(true);
                if is_important && !applied {
                    match ConfigurationDisconnect::new(
                        "Important Resource Pack got not applied".to_string(),
                    )
                    .write(writer)
                    .await
                    {
                        Ok(()) => (),
                        Err(_) => {
                            return Err((
                                ConnectionHandlerError::PacketSent("".to_string()),
                                player.clone(),
                            ))
                        }
                    }
                }
            }
            ServerboundPackets::ConfigurationPluginMessage(_message) => {
                let channel = _message.channel.get_value();
                let data = _message.data.get_value();
                let _ = sender
                    .send(ServerMessage::PluginMessage(player.clone(), channel, data))
                    .await;
            }
        }
        Ok(())
    }
    /// function to handle the connection
    ///
    /// # Arguments
    /// `stream` - A mutable reference to the `TcpStream` of the connection
    /// `sender` - A `Sender<ServerMessage>` to be able to communicate with the server
    /// `settings` - A instance of the `ServerSettings`
    pub(crate) async fn run(
        stream: &mut TcpStream,
        sender: mpsc::Sender<ServerMessage>,
        settings: ServerSettings,
    ) -> Result<(), (ConnectionHandlerError, Option<Player>)> {
        println!("Connected");
        let mut player: Option<Player> = None;
        let mut state = State::Handshake;
        let (player_sender, mut player_receiver) = mpsc::channel::<PlayerMessages>(20);
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        let mut verify_token = [0; 4];
        let mut last_keep_alive_send = Instant::now();
        let mut keep_alive_answered_at = Instant::now();
        let mut await_keep_alive_answer = false;
        rand::thread_rng().fill_bytes(&mut verify_token);
        let mut last_checked = Instant::now();
        let update_time = tokio::time::Duration::from_secs(1).div_f64(100.0);
        let mut packet_queue: VecDeque<fast_protocol::datatypes::packets::ClientboundPackets> =
            VecDeque::new();
        loop {
            println!("Packet amount: {}", packet_queue.len());
            if let Some(packet) = packet_queue.pop_front() {
                packet.write(&mut writer).await;
            }
            Self::send_keep_alive(
                &state,
                &mut writer,
                &mut last_keep_alive_send,
                &keep_alive_answered_at,
                &mut await_keep_alive_answer,
                &player,
            )
            .await?;
            Self::receive_packets(
                &mut reader,
                &mut writer,
                &sender,
                &mut state,
                &mut player,
                &settings,
                &mut keep_alive_answered_at,
                &mut await_keep_alive_answer,
                &player_sender,
                &mut packet_queue,
            )
            .await?;
            Self::handle_incomming_messages(&mut writer, &mut player_receiver, &mut packet_queue)
                .await;
            let now = Instant::now();
            let sleep_time = now.duration_since(last_checked).saturating_sub(update_time);
            last_checked = now;
            tokio::time::sleep(sleep_time).await;
        }
    }
}
async fn fetch_url(url: &str) -> reqwest::Result<String> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
