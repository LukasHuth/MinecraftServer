use fast_protocol::datatypes::packets::configuration::serverbound::{ChatMode, MainHand};
use tokio::sync::mpsc;

/// Enum used for P2P (player to player) communication
pub enum PlayerMessages {
}

/// Player information
#[derive(Clone)]
pub struct PlayerInformation {
    /// language that the player uses
    pub locale: String,
    /// view distance of the user
    pub view_distance: u8,
    /// chat mode that the player uses
    pub chat_mode: ChatMode,
    /// whether the player wants chat colors or not
    pub chat_colors: bool,
    /// mask of the displayed skin parts
    pub displayed_skin_parts: u8,
    /// main hand of the player
    pub main_hand: MainHand,
    /// whether the player wants to have text filtering or not
    pub text_filtering: bool,
    /// whether the player wants to appear in the server list or not
    pub in_server_listing: bool,
}
/// struct with all player data
#[derive(Clone)]
pub struct Player {
    /// uuid of the player
    pub uuid: u128,
    /// username of the player
    pub username: String,
    /// channel that can be used to send data to the player
    pub sender: mpsc::Sender<PlayerMessages>,
    /// optional `PlayerInformation`
    ///
    /// # Note
    ///
    /// This field is only optional, because the player gets cvreated, before he sends all his
    /// information, after the `Configuration` state is over it should always contain data
    pub information: Option<PlayerInformation>
}
