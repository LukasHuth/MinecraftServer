use datatypes::ImportantFunctions;
use nbt_lib::NbtValue;

use crate::datatypes::datatype_definition::{self as custom_datatypes, TextComponent};

use super::TagArrayData;
impl super::FeatureFlags {
    pub fn new(flags: Vec<datatypes::Identifier>) -> Self {
        let feature_count = datatypes::VarInt::new(flags.len() as i32);
        Self { feature_count, feature_flags: datatypes::Array::new(flags) }
    }
}
impl super::AddResourcePack {
    pub fn new(uuid: u128, url: String, hash: String, forced: bool, prompt_message: Option<custom_datatypes::TextComponent>) -> Self {
        Self {
            uuid: datatypes::UUID::new(uuid),
            url: datatypes::String::new(url),
            hash: datatypes::String::new(hash),
            forced,
            has_prompt_message: prompt_message
        }
    }
}
impl super::RemoveResoucePack {
    pub fn new(uuid: Option<u128>) -> Self {
        Self { has_uuid: uuid.is_some(), uuid: uuid.map(|e| datatypes::UUID::new(e)) }
    }
}
impl super::RegestryData {
    pub fn new(data: NbtValue) -> Self {
        Self { regestry_codec: custom_datatypes::NBT::from(data) }
    }
}
impl super::FinishConfiguration {
    pub fn new() -> Self { Self }
}
impl super::KeepAlive {
    pub fn new(id: i64) -> Self {
        Self { id: datatypes::Long::new(id) }
    }
}
impl super::UpdateTags {
    pub fn new(data: Vec<TagArrayData>) -> Self {
        Self { tag_array: datatypes::Array::new(data) }
    }
}
impl super::TagArrayData {
    pub fn new(tag_name: datatypes::Identifier, entries: Vec<i32>) -> Self {
        Self { tag_name, entries: datatypes::Array::new(entries.iter().map(|&e|datatypes::VarInt::new(e)).collect()) }
    }
}
impl super::ConfigurationDisconnect {
    pub fn new(message: String) -> Self {
        Self { reason: TextComponent::from(message) }
    }
}
impl super::ClientboundPluginMessage {
    pub fn new(channel: datatypes::Identifier, data: Vec<u8>) -> Self {
        Self { channel, data: datatypes::ByteArray::new(data) }
    }
}
