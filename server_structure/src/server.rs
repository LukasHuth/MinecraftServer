use std::net::TcpListener;

use uuid::Uuid;

use crate::player::Player;

use self::config::Configuration;

pub mod config;

#[derive(Clone)]
pub struct ServerSettings {
    pub version: &'static str,
    pub motd: String,
    pub max_players: u16,
    pub protocol_version: u16,
    pub offline_mode: bool,
    pub port: u16,
}
impl ServerSettings {
    pub fn new(config: config::Configuration) -> Self {
        Self {
            max_players: config.max_players,
            motd: config.motd.clone(),
            version: config::SERVER_VERSION,
            protocol_version: config::PROTOCOL_VERSION,
            offline_mode: config.offline_mode,
            port: config.port,
        }
    }
}
pub struct Server {
    pub players: Vec<Player>,
    pub server_settings: ServerSettings,
}
pub enum ServerEvent {
    PlayerJoined(Player),
    Broadcast(String),
}
#[allow(dead_code)]
pub enum HandleEvent {}
impl Server {
    pub fn new() -> Self {
        let config = Configuration::load();
        Self {players: Vec::new(), server_settings: ServerSettings::new(config) }
    }
    #[allow(dead_code)]
    pub fn handle(&self, _event: HandleEvent) {
    }
    pub fn get_players(&self) -> Vec<Player> {
        self.players.clone()
    }
    pub fn get_player_handles(&mut self) -> Vec<&mut Player> {
        self.players.iter_mut().collect()
    }
    pub fn get_player(&self, uuid: Uuid) -> &Player {
        self.players.iter()
            .filter(|p|p.uuid.to_uuid().eq(&uuid)).next().expect("")
    }
    pub fn remove_player(&mut self, player: packet_api::datatypes::datastructs::player::Player) {
        self.players.retain(|p|p.uuid != player.uuid);
    }
    pub fn trigger(&mut self, event: ServerEvent) {
        match event {
            ServerEvent::PlayerJoined(player) => {
                let player_username = player.username.clone();
                self.players.push(player);
                self.trigger(ServerEvent::Broadcast(format!("{} joined the game", player_username)));
            },
            ServerEvent::Broadcast(message) => {
                println!("{message}");
                for player in self.get_player_handles() {
                    player.send_message(&message);
                }
            }
        }
    }
    pub fn start_server() -> (Server, TcpListener) {
        let server = Server::new();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", server.server_settings.port)).unwrap();
        println!("Starting server on port {}", server.server_settings.port);
        (server, listener)
    }
}
