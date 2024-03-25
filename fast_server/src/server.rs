use self::server_settings::ServerSettings;

use crate::{player::Player, connection_handler::ConnectionHandler};

use openssl::pkey::Private;
use tokio::{net::TcpListener, sync::{mpsc, oneshot}};

pub(crate) mod server_settings;
pub struct Server {
    pub players: Vec<Player>,
    pub settings: ServerSettings,
    pub rsa_key: openssl::rsa::Rsa<Private>,
}
pub(crate) enum ServerMessage {
    RemovePlayer(u128),
    GetPlayers(oneshot::Sender<Vec<Player>>),
    GetPlayerAmount(oneshot::Sender<u16>),
    AddPlayer(Option<Player>),
}
impl Server {
    fn new(settings: ServerSettings) -> Self {
        let rsa_key = openssl::rsa::Rsa::generate(1024).unwrap();
        Self { players: Vec::new(), settings, rsa_key }
    }
    async fn server_manager(server_settings: ServerSettings, mut receiver: mpsc::Receiver<ServerMessage>) {
        let mut server = Server::new(server_settings);
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
            }
        }
    }
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server_settings = ServerSettings::default();
        let (sender, receiver) = mpsc::channel::<ServerMessage>(32);
        let _manager_task = tokio::spawn(Server::server_manager(server_settings.clone(), receiver));
        let address = format!("0.0.0.0:{}", server_settings.port);
        let listener = TcpListener::bind(address).await?;
        println!("Starting server on port {}", server_settings.port);
        loop {
            if let Ok((socket, _)) = listener.accept().await {
                let sender_clone = sender.clone();
                let server_settings_clone = server_settings.clone();
                tokio::spawn(async move {
                    if let Err(err) = ConnectionHandler::run(socket, sender_clone.clone(), server_settings_clone).await {
                        if err.1.is_some() {
                            let player = err.1.unwrap();
                            let _ = sender_clone.send(ServerMessage::RemovePlayer(player.uuid)).await;
                            eprintln!("Error handling connection: {:?}", err.0);
                        }
                        // if the player is none the player wasn't connected yet so the error is
                        // unneccessary
                    }
                });
            }
        }
    }
}
