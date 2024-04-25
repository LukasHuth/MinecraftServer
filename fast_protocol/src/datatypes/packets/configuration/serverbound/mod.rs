mod implementations;
mod important_functions;
/// Enum of all the available Chat modes
#[derive(Clone, Copy)]
pub enum ChatMode {
    /// Everything should be send to the client
    Enabled,
    /// Only commmand data should get send to the player
    CommandsOnly,
    /// nothing from the chat should get send to the player
    Hidden,
}
/// Enum of the main hand options
#[derive(Clone, Copy)]
pub enum MainHand {
    /// Left hand
    Left,
    /// Right hand
    Right,
}
/// Enum of the different responses to an resource pack packet
#[derive(Clone, Copy)]
pub enum ResourcePackResponseEnum {
    /// The client was able to download the resource pack
    SuccessfullyDownloaded,
    /// The client decliened to add the resource pack
    Declined,
    /// The client was not able to download the resource pack
    FailedToDownload,
    /// the Resource pack request was accepted
    Accepted,
    /// The resourcepack was already downloaded
    Downloaded,
    /// The client could not establish a connection to the download url
    InvalidUrl,
    /// The client was unable to reload its resource packs
    FailedToReload,
    /// The downloaded resource pack was discarded, because the hash was not correct
    Discarded,
}

/// Packet containg every important information of the client
pub struct ClientInformation {
    /// language of the client
    pub locale: datatypes::String,
    /// View distance of the client
    pub view_distance: datatypes::Byte,
    /// which `ChatMode` the client is using
    pub chat_mode: datatypes::Enum<ChatMode, datatypes::VarInt>,
    /// if the client wants to use chat colors
    pub chat_colors: datatypes::Boolean,
    /// mask of the displayed skin parts
    pub displayed_skin_parts: datatypes::UnsignedByte,
    /// selection which hand the client uses as his `MainHand`
    pub main_hand: datatypes::Enum<MainHand, datatypes::VarInt>,
    /// selection about the chat filter
    pub text_filtering: datatypes::Boolean,
    /// selection, if the username is allowed to be displayed on the serverlist
    pub allow_server_listing: datatypes::Boolean,
}
/// Packet for plugin messages
pub struct ServerboundPluginMessage {
    /// channel name
    pub channel: datatypes::Identifier,
    /// channel data
    pub data: datatypes::ByteArray,
}
/// Packet to tell the server that `FinishConfiguration` was received
pub struct AckFinishConfiguration;
/// Response to the `KeepAlive`
pub struct KeepAliveResponse {
    /// id if the received `KeepAlive`
    pub id: datatypes::Long
}
/// this is a packet, defined in the documentation, but not used by default, so there is no
/// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
/// deprecated)
#[deprecated]
pub struct Pong {}
/// Packet that answers, how it proceded after the `AddResoucePack` packet
pub struct ResoucePackResponse {
    /// id of the requested resource pack
    pub uuid: datatypes::UUID,
    /// result, how the request went
    pub result: datatypes::Enum<ResourcePackResponseEnum, datatypes::VarInt>,
}
