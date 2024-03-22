use serde::Deserialize;

pub mod constants;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
pub struct Config {
    pub offline_mode: bool,
    pub max_players: u16,
    pub motd: String,
    pub port: u16,
}
impl Config {
    pub(crate) fn load() -> Self {
        let content = std::fs::read_to_string("Config.toml").unwrap();
        let data: Data = toml::from_str(&content).unwrap();
        data.config
    }
}
