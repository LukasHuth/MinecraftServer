use binary_utils::{DataWriter, DataReader};
use datatypes::{Identifier, ByteArray, Long, UUID, VarInt, Array, String, Byte, Enum, GetU64, ImportantEnumTrait, UnsignedByte};
use tokio::io::{AsyncWrite, AsyncRead};

use crate::datatypes::datatype_definition::{TextComponent, NBT};

type StdString = std::string::String;

// clientbound s -> c
pub struct ClientboundPluginMessage {
    pub channel: Identifier,
    pub data: ByteArray,
}
pub struct Disconnect {
    pub reason: TextComponent,
}
pub struct FinishConfiguration;
pub struct KeepAlive {
    pub id: Long,
}
// INFO: this is a packet, defined in the documentation, but not used by default, so there is no
// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
// deprecated
#[deprecated]
pub struct Ping {}
pub struct RegestryData {
    pub regestry_codec: NBT
}
pub struct RemoveResoucePack {
    pub has_uuid: bool,
    pub uuid: Option<UUID>,
}
pub struct AddResourcePack {
    pub uuid: UUID,
    pub url: StdString,
    pub hash: StdString,
    pub forced: bool,
    pub has_prompt_message: Option<TextComponent>,
}
pub struct FeatureFlags {
    pub feature_count: VarInt,
    pub feature_flags: Identifier,
}
pub struct TagArrayData {
    // pub length: VarInt
    pub tag_name: Identifier,
    // pub count: VarInt,
    pub entries: Array<VarInt>,
}
// https://wiki.vg/Protocol#Update_Tags_.28configuration.29
pub struct UpdateTags {
    // pub ArrayLength: VarInt
    pub tag_array: Array<TagArrayData>
}
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
pub enum MainHand {
    Left,
    Right,
}
// serverbound s->c
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: Byte,
    pub chat_mode: Enum<ChatMode, VarInt>,
    pub chat_colors: bool,
    pub displayed_skin_parts: UnsignedByte,
    pub main_hand: Enum<MainHand, VarInt>,
    pub text_filtering: bool,
    pub allow_server_listings: bool,
}
pub struct ServerboundPluginMessage {
    pub channel: Identifier,
    pub data: ByteArray,
}
pub struct AckFinishConfiguration;
pub struct KeepAliveResponse {
    pub id: Long
}
// INFO: this is a packet, defined in the documentation, but not used by default, so there is no
// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
// deprecated
#[deprecated]
pub struct Pong {}
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
pub struct ResoucePackResponse {
    pub uiid: UUID,
    pub result: Enum<ResourcePackResponseEnum, VarInt>,
}
impl ImportantEnumTrait for ChatMode {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            3..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl ImportantEnumTrait for MainHand {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            2..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl ImportantEnumTrait for ResourcePackResponseEnum {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(Self::SuccessfullyDownloaded),
            1 => Ok(Self::Declined),
            2 => Ok(Self::FailedToDownload),
            3 => Ok(Self::Accepted),
            4 => Ok(Self::Downloaded),
            5 => Ok(Self::InvalidUrl),
            6 => Ok(Self::FailedToReload),
            7 => Ok(Self::Discarded),
            8..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl DataWriter for TagArrayData {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
impl DataReader for TagArrayData {
    async fn read(_reader: &mut (impl AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        todo!()
    }
}
// TODO: implement read and write
