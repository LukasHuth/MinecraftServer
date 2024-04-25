use self::server_settings::ServerSettings;

use crate::{player::Player, connection_handler::ConnectionHandler};

use nbt_lib::datatypes::TextComponent;
use openssl::pkey::Private;
use tokio::{net::TcpListener, sync::{mpsc, oneshot}, io::AsyncWriteExt};

pub(crate) mod server_settings;
/// Struct to store the players, settings and to handle/ update the world
///
/// # Note
///
/// This Struct is not finished, but will be managing the minecraft world
pub struct Server {
    /// A list of all connected players
    pub players: Vec<Player>,
    /// the settings of the server
    pub settings: ServerSettings,
    /// the private key used for the authentication of the players
    pub rsa_key: openssl::rsa::Rsa<Private>,
}
/// Struct for managing server messages
pub(crate) enum ServerMessage {
    RemovePlayer(u128),
    GetPlayers(oneshot::Sender<Vec<Player>>),
    GetPlayerAmount(oneshot::Sender<u16>),
    AddPlayer(Option<Player>),
    IsResourcePackImportant(datatypes::UUID, oneshot::Sender<bool>),
    PluginMessage(Option<Player>, String, Vec<u8>),
    GetConfigData(oneshot::Sender<Vec<ConfigData>>),
}
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) enum ConfigData {
    PluginMessage(String, Vec<u8>),
    RegistryData(nbt_lib::NbtValue),
    RemoveResourcePack(Option<u128>),
    AddResourcePack(u128, String, String, bool, Option<TextComponent>),
    FeatureFlags(Vec<String>),
    // TODO: Update Tags
}
impl Server {
    fn new(settings: ServerSettings) -> Self {
        let rsa_key = openssl::rsa::Rsa::generate(1024).unwrap();
        Self { players: Vec::new(), settings, rsa_key }
    }
    async fn server_manager(server_settings: ServerSettings, mut receiver: mpsc::Receiver<ServerMessage>) {
        let mut server = Server::new(server_settings);
        let config = vec![
            // ConfigData::FeatureFlags(vec!["minecraft:vanilla".to_string(), "minecraft:bundle".to_string(), "minecraft:trade_rebalance".to_string()])
        ];
        while let Some(message) = receiver.recv().await {
            match message {
                ServerMessage::RemovePlayer(uuid) => {
                    server.players.retain(|p| p.uuid != uuid);
                }
                ServerMessage::GetPlayers(sender) => {
                    let _ = sender.send(server.players.clone());
                }
                ServerMessage::GetPlayerAmount(sender) => {
                    let _ = sender.send(server.players.len() as u16);
                }
                ServerMessage::AddPlayer(player) => {
                    if let Some(player) = player {
                        server.players.push(player);
                    }
                }
                ServerMessage::IsResourcePackImportant(_uuid, sender) => {
                    // TODO: Implement a real check
                    let _ = sender.send(false);
                }
                ServerMessage::PluginMessage(_player, _channel, _data) => {
                    // TODO: do something with the plugin message
                }
                ServerMessage::GetConfigData(sender) => {
                    let _ = sender.send(config.clone());
                }
            }
        }
    }
    /// This functions starts a server on the port specified in the configs and only returns a
    /// value if the server closes or an error occured that forces the server to stop
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server_settings = ServerSettings::default();
        let (sender, receiver) = mpsc::channel::<ServerMessage>(32);
        let _manager_task = tokio::spawn(Server::server_manager(server_settings.clone(), receiver));
        let address = format!("0.0.0.0:{}", server_settings.port);
        let listener = TcpListener::bind(address).await?;
        println!("Starting server on port {}", server_settings.port);
        loop {
            if let Ok((mut socket, _)) = listener.accept().await {
                let sender_clone = sender.clone();
                let server_settings_clone = server_settings.clone();
                tokio::spawn(async move {
                    println!("New thread created");
                    if let Err(err) = ConnectionHandler::run(&mut socket, sender_clone.clone(), server_settings_clone).await {
                        if err.1.is_some() {
                            let player = err.1.unwrap();
                            let _ = sender_clone.send(ServerMessage::RemovePlayer(player.uuid)).await;
                            eprintln!("Error handling connection: {:?}", err.0);
                        }
                        // if the player is none the player wasn't connected yet so the error is
                        // unneccessary
                    }
                    println!("Closing TCP Connection");
                    if let Err(e) = socket.shutdown().await {
                        eprintln!("Could not close the tcp stream: {e}");
                    }
                });
            }
        }
    }
}
