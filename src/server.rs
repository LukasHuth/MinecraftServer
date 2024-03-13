use packet_api::datatypes::datastructs::Player;

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
#[allow(dead_code)]
pub enum HandleEvent {}
impl Server {
    pub fn new() -> Self {
        Self {players: vec![], server_settings: ServerSettings::new() }
    }
    #[allow(dead_code)]
    pub fn handle(&self, _event: HandleEvent) {
    }
}
