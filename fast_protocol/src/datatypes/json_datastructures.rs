use binary_utils::DataWriter;
use datatypes::ImportantFunctions;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncWrite;

#[derive(Serialize, Deserialize)]
pub(crate) struct Version {
    name: std::string::String,
    protocol: u16,
}
/// This struct represents a basic player in the minecraft protocol
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
    text: String,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct StatusResponseJSON {
    version: Version,
    players: Players,
    description: Description,
    favicon: String,
    #[serde(rename = "enforcesSecureChat")]
    enforces_secure_chat: bool,
    #[serde(rename = "previewsChar")]
    previews_char: bool,
}
impl Player {
    /// Function to create a new instance of `Player`
    ///
    /// # Arguments
    ///
    /// `name` - name of the player
    /// `id` - uuid of the player as a string
    pub fn new(name: std::string::String, id: std::string::String) -> Self {
        Self {name, id}
    }
}
impl StatusResponseJSON {
    pub(crate) fn new(version: String, protocol: u16, max_players: u16, player_count: u16, players: Vec<Player>, modt: String, image: String) -> Self {
        let version = Version { name: version, protocol };
        let players = Players { max: max_players, online: player_count, sample: players };
        let description = Description { text: modt };
        Self { version, players, description, favicon: image, enforces_secure_chat: true, previews_char: true }
    }
}
impl DataWriter for StatusResponseJSON {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let data = datatypes::String::new(match serde_json::to_string(self) { Ok(v) => Ok(v), Err(_) => binary_utils::Error::FailedToWrite.into()}?);
        data.write(writer).await?;
        Ok(())
    }
}
