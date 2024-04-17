use datatypes::ImportantFunctions;
use nbt_lib::NbtValue;

use nbt_lib::datatypes::{TextComponent, NBT};

use super::TagArrayData;
impl super::FeatureFlags {
		/// function to initialize a new instance of `FeatureFlags`
		/// 
		/// # Arguments
    /// `flags` - Identifiers of the flags to set
    pub fn new(flags: Vec<datatypes::Identifier>) -> Self {
        let feature_count = datatypes::VarInt::new(flags.len() as i32);
        Self { feature_count, feature_flags: datatypes::Array::new(flags) }
    }
}
impl super::AddResourcePack {
		/// function to initialize a new instance of `AddResourcePack`
		/// 
		/// # Arguments
    /// `uuid` - uuid of the resource pack
    /// `url` - url where the resource pack can be downloaded from
    /// `hash` - SHA-1 hash of the resource pack
    /// `forced` - if the resource pack is forced or not
    /// `prompt_message` - Optional message to display for the resource pack
    pub fn new(uuid: u128, url: String, hash: String, forced: bool, prompt_message: Option<TextComponent>) -> Self {
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
		/// function to initialize a new instance of `RemoveResoucePack`
		/// 
		/// # Arguments
    /// `uuid` - Optional uuid of the resource pack, if None all will be removed
    pub fn new(uuid: Option<u128>) -> Self {
        Self { has_uuid: uuid.is_some(), uuid: uuid.map(|e| datatypes::UUID::new(e)) }
    }
}
impl super::RegistryData {
		/// function to initialize a new instance of `RegistryData`
		/// 
		/// # Arguments
    /// `data` - registry data that should get updated
    pub fn new(data: NbtValue) -> Self {
        Self { regestry_codec: NBT::from(data) }
    }
}
impl super::FinishConfiguration {
		/// function to initialize a new instance of `FinishConfiguration`
    pub fn new() -> Self { Self }
}
impl super::KeepAlive {
		/// function to initialize a new instance of `KeepAlive`
		/// 
		/// # Arguments
    /// `id` - Id of the packet
    pub fn new(id: i64) -> Self {
        Self { id: datatypes::Long::new(id) }
    }
}
impl super::UpdateTags {
		/// function to initialize a new instance of `UpdateTags`
		/// 
		/// # Arguments
    /// `data` - list of `TagArrayData`
    pub fn new(data: Vec<TagArrayData>) -> Self {
        Self { tag_array: datatypes::Array::new(data) }
    }
}
impl super::TagArrayData {
		/// function to initialize a new instance of `TagArrayData`
		/// 
		/// # Arguments
    /// `tag_name` - The name of the tag
    /// `entries` - data of the tag
    pub fn new(tag_name: datatypes::Identifier, entries: Vec<i32>) -> Self {
        Self { tag_name, entries: datatypes::Array::new(entries.iter().map(|&e|datatypes::VarInt::new(e)).collect()) }
    }
}
impl super::ConfigurationDisconnect {
		/// function to initialize a new instance of `ConfigurationDisconnect`
		/// 
		/// # Arguments
    /// `message` - Reason of the disconnect
    pub fn new(message: String) -> Self {
        Self { reason: TextComponent::from(message) }
    }
}
impl super::ConfigurationClientboundPluginMessage {
		/// function to initialize a new instance of `ConfigurationClientboundPluginMessage`
		/// 
		/// # Arguments
    /// `channel` - Channel name
    /// `data` - data
    pub fn new(channel: datatypes::Identifier, data: Vec<u8>) -> Self {
        Self { channel, data: datatypes::ByteArray::new(data) }
    }
}
