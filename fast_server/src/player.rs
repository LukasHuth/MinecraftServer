use fast_protocol::datatypes::packets::{ChatMode, MainHand};
use tokio::sync::mpsc;

pub enum PlayerMessages {
}

#[derive(Clone)]
pub struct PlayerInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub text_filtering: bool,
    pub in_server_listing: bool,
}
#[derive(Clone)]
pub struct Player {
    pub uuid: u128,
    pub username: String,
    pub sender: mpsc::Sender<PlayerMessages>,
    pub information: Option<PlayerInformation>
}
