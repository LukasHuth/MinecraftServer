use std::{sync::{Arc, Mutex}, cell::RefCell};

use self::server_settings::ServerSettings;

use crate::{player::Player, connection_handler::{ConnectionHandler, self, ConnectionHandlerError}};

use tokio::sync::mpsc;
use tokio::net::TcpListener;

mod server_settings;
#[derive(Clone)]
pub struct Server {
    pub players: Vec<Player>,
    pub settings: ServerSettings,
}
impl Server {
    fn new() -> Self {
        let settings = ServerSettings::default();
        Self { players: Vec::new(), settings }
    }
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server = Server::new();
        let address = format!("0.0.0.0:{}", server.settings.port);
        let listener = TcpListener::bind(address).await?;
        println!("Starting server on port {}", server.settings.port);
        let server_arc = Arc::new(server);
        
        loop {
            let (socket, _) = listener.accept().await?;
            let server_clone = Arc::clone(&server_arc);
            tokio::spawn(async move {
                if let Err(err) = ConnectionHandler::run(socket, server_clone).await {
                    eprintln!("Error handling connection: {:?}", err);
                }

            });
        }
    }
}
