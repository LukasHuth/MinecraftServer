use uuid::Uuid;

use crate::player::Player;

const SERVER_VERSION: &str = "1.20.4";
const MOTD: &str = "A Cool Rust Server";
const MAX_PLAYERS: u16 = 100;
const PROTOCOL_VERSION: u16 = 765;
const OFFLINE_MODE: bool = true;

#[derive(Clone)]
pub struct ServerSettings {
    pub version: &'static str,
    pub motd: &'static str,
    pub max_players: u16,
    pub protocol_version: u16,
    pub offline_mode: bool,
}
impl ServerSettings {
    pub fn new() -> Self {
        Self {
            max_players: MAX_PLAYERS,
            motd: MOTD,
            version: SERVER_VERSION,
            protocol_version: PROTOCOL_VERSION,
            offline_mode: OFFLINE_MODE,
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
        Self {players: Vec::new(), server_settings: ServerSettings::new() }
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
}
