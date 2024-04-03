mod implementations;
mod important_functions;
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
pub enum MainHand {
    Left,
    Right,
}
pub enum ResourcePackResponseEnum {
    SuccessfullyDownloaded,
    Declined,
    FailedToDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedToReload,
    Discarded,
}

pub struct ClientInformation {
    pub locale: datatypes::String,
    pub view_distance: datatypes::Byte,
    pub chat_mode: datatypes::Enum<ChatMode, datatypes::VarInt>,
    pub chat_colors: datatypes::Boolean,
    pub displayed_skin_parts: datatypes::UnsignedByte,
    pub main_hand: datatypes::Enum<MainHand, datatypes::VarInt>,
    pub text_filtering: datatypes::Boolean,
    pub allow_server_listings: datatypes::Boolean,
}
pub struct ServerboundPluginMessage {
    pub channel: datatypes::Identifier,
    pub data: datatypes::ByteArray,
}
pub struct AckFinishConfiguration;
pub struct KeepAliveResponse {
    pub id: datatypes::Long
}
// INFO: this is a packet, defined in the documentation, but not used by default, so there is no
// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
// deprecated
#[deprecated]
pub struct Pong {}
pub struct ResoucePackResponse {
    pub uiid: datatypes::UUID,
    pub result: datatypes::Enum<ResourcePackResponseEnum, datatypes::VarInt>,
}
