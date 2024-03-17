use serde::Deserialize;

pub const SERVER_VERSION: &str = "1.20.4";
pub const PROTOCOL_VERSION: u16 = 765;

#[derive(Deserialize)]
struct Data {
    configuration: Configuration,
}
#[derive(Deserialize)]
pub struct Configuration {
    pub offline_mode: bool,
    pub max_players: u16,
    pub motd: String,
    pub port: u16,
}
impl Configuration {
    pub(crate) fn load() -> Configuration {
        let content = std::fs::read_to_string("config.toml").unwrap();
        let data: Data = toml::from_str(&content).unwrap();
        data.configuration
    }
}
