mod implementations;
mod important_functions;
#[derive(Clone, Copy)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
#[derive(Clone, Copy)]
pub enum MainHand {
    Left,
    Right,
}
#[derive(Clone, Copy)]
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
    pub allow_server_listing: datatypes::Boolean,
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
    pub uuid: datatypes::UUID,
    pub result: datatypes::Enum<ResourcePackResponseEnum, datatypes::VarInt>,
}
