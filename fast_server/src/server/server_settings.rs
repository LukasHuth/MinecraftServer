use crate::config;

#[derive(Clone)]
pub struct ServerSettings {
    pub version: &'static str,
    pub motd: String,
    pub max_players: u16,
    pub protocol_version: u16,
    pub offline_mode: bool,
    pub port: u16,
}
impl Default for ServerSettings {
    fn default() -> Self {
        let config = config::Config::load();
        Self {
            max_players: config.max_players,
            motd: config.motd,
            offline_mode: config.offline_mode,
            port: config.port,
            version: config::constants::SERVER_VERSION,
            protocol_version: config::constants::PROTOCOL_VERSION,
        }
    }
}
