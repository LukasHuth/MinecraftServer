use serde::{Serialize, Deserialize};
use tokio::io::AsyncWrite;

use crate::utils::DataWriter;
use crate::errors::Error;
use super::datatype_definition::{String, ImportantFunctions as _};

#[derive(Serialize, Deserialize)]
pub(crate) struct Version {
    name: std::string::String,
    protocol: u16,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: std::string::String,
    id: std::string::String,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Players {
    max: u16,
    online: u16,
    sample: Vec<Player>,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Description {
    text: std::string::String,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponseJSON {
    version: Version,
    players: Players,
    description: Description,
    favicon: std::string::String,
    #[serde(rename = "enforcesSecureChat")]
    enforces_secure_chat: bool,
    #[serde(rename = "previewsChar")]
    previews_char: bool,
}
impl StatusResponseJSON {
    pub(crate) fn new(version: std::string::String, protocol: u16, max_players: u16, player_count: u16, players: Vec<Player>, modt: std::string::String, image: std::string::String) -> Self {
        let version = Version { name: version, protocol };
        let players = Players { max: max_players, online: player_count, sample: players };
        let description = Description { text: modt };
        Self { version, players, description, favicon: image, enforces_secure_chat: true, previews_char: true }
    }
}
impl DataWriter for StatusResponseJSON {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        let data = String::new(match serde_json::to_string(self) { Ok(v) => Ok(v), Err(_) => Error::FailedToWrite.into()}?);
        data.write(writer).await?;
        Ok(())
    }
}
