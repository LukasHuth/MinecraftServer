use serde::Deserialize;

/// Module to hold every constant that is needed by the `Config`
pub mod constants;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
/// Struct of the server config
#[derive(Deserialize)]
pub struct Config {
    /// whether the server should operate in the offline mode or not
    pub offline_mode: bool,
    /// the amount of players allowed to join the server
    pub max_players: u16,
    /// the motd of the server
    pub motd: String,
    /// the port that the server is runnning on
    pub port: u16,
}
impl Config {
    pub(crate) fn load() -> Self {
        let content = std::fs::read_to_string("Config.toml").unwrap();
        let data: Data = toml::from_str(&content).unwrap();
        data.config
    }
}
