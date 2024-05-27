//! This module contains all clientbound configuration packets
use nbt_lib::datatypes::{TextComponent, NBT};

mod implementations;
mod new_impl;
/// Plugin message used during the `Configuration` phase
pub struct ConfigurationClientboundPluginMessage {
    /// channel name that the plugin message wants to send data to
    pub channel: datatypes::Identifier,
    /// the data that should be send to the channel
    pub data: datatypes::ByteArray,
}
/// Disconnect packet to disconnect the client durent the `Configuration` phase
pub struct ConfigurationDisconnect {
    /// reason why the client gets disconnected
    pub reason: TextComponent,
}
/// Packet to tell the client that the configuration is finished and the next state `Playing` is
/// selected after the acknowledgment
pub struct FinishConfiguration;
/// Keep Alive packet
///
/// # Note
///
/// This packet has to be send every 20 seconds and the client has to be disconnected, if no asnwer
/// is received after 15 seconds
pub struct KeepAlive {
    /// if that the response has to include
    pub id: datatypes::Long,
}
/// this is a packet, defined in the documentation, but not used by default, so there is no
/// need currently to implement it, but reservated, if it will be needed. (thats why it's marked as
/// deprecated)
#[deprecated]
pub struct Ping {}
/// This packet is used to alter registry data of minecraft to allow more ways to customize the
/// game
pub struct RegistryData {
    /// the regestry data to alter
    pub regestry_codec: NBT
}
/// Packet to request remove one or all resourcepacks
pub struct RemoveResoucePack {
    /// boolean to set to true if a specific resource pack with a specific uuid should be removed
    /// or false if every resource pack should be removed
    pub has_uuid: bool,
    /// optional uuid of the resource pack that should get removed
    pub uuid: Option<datatypes::UUID>,
}
/// Packet to tell the client that it should add a resource pack
pub struct AddResourcePack {
    /// uuid of the resource pack
    pub uuid: datatypes::UUID,
    /// url, where the resource pack should be downloaded
    pub url: datatypes::String,
    /// hash of the resource pack to enshure that the right one was downloaded
    pub hash: datatypes::String,
    /// bool if the resource pack is forced
    pub forced: bool,
    /// optional message that should be displayed for the resource pack
    pub has_prompt_message: Option<TextComponent>,
}
/// Packet to enable certain features of the client
pub struct FeatureFlags {
    /// the amount of set features
    pub feature_count: datatypes::VarInt,
    /// list of all features
    pub feature_flags: datatypes::Array<datatypes::Identifier>,
}
/// Data struct used in `UpdateTags`
#[derive(Clone)]
pub struct TagArrayData {
    // pub length: VarInt
    /// tag name that should be updated
    pub tag_name: datatypes::Identifier,
    // pub count: VarInt,
    /// list of entries
    pub entries: datatypes::Array<datatypes::VarInt>,
}
// https://wiki.vg/Protocol#Update_Tags_.28configuration.29
/// Packet to update certain tags
pub struct UpdateTags {
    // pub ArrayLength: VarInt
    /// list of the tag data `TagArrayData`
    pub tag_array: datatypes::Array<TagArrayData>
}
