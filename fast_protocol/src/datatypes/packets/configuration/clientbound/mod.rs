use nbt_lib::datatypes::{TextComponent, NBT};

mod implementations;
mod new_impl;
pub struct ConfigurationClientboundPluginMessage {
    pub channel: datatypes::Identifier,
    pub data: datatypes::ByteArray,
}
pub struct ConfigurationDisconnect {
    pub reason: TextComponent,
}
pub struct FinishConfiguration;
pub struct KeepAlive {
    pub id: datatypes::Long,
}
// INFO: this is a packet, defined in the documentation, but not used by default, so there is no
// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
// deprecated
#[deprecated]
pub struct Ping {}
pub struct RegistryData {
    pub regestry_codec: NBT
}
pub struct RemoveResoucePack {
    pub has_uuid: bool,
    pub uuid: Option<datatypes::UUID>,
}
pub struct AddResourcePack {
    pub uuid: datatypes::UUID,
    pub url: datatypes::String,
    pub hash: datatypes::String,
    pub forced: bool,
    pub has_prompt_message: Option<TextComponent>,
}
pub struct FeatureFlags {
    pub feature_count: datatypes::VarInt,
    pub feature_flags: datatypes::Array<datatypes::Identifier>,
}
#[derive(Clone)]
pub struct TagArrayData {
    // pub length: VarInt
    pub tag_name: datatypes::Identifier,
    // pub count: VarInt,
    pub entries: datatypes::Array<datatypes::VarInt>,
}
// https://wiki.vg/Protocol#Update_Tags_.28configuration.29
pub struct UpdateTags {
    // pub ArrayLength: VarInt
    pub tag_array: datatypes::Array<TagArrayData>
}
