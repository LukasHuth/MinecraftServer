//! A module for all clientbound packets that are beeing send while playing
//!
//! All of them are defined here:
//! <https://wiki.vg/Protocol#Clientbound_5>

/// The delimiter for a bundle of packets.
///
/// It ensures that every packet after it until getting the
/// of is only stored and will be executed at the same time, it is for example used to spawn every
/// entity of the same type on the same tick
pub struct BundleDelimiter;
mod spawn_entity;
use datatypes::{
    Array, Boolean, Byte, ByteArray, Double, FixedPoint, Identifier, Int, Long, Position, Short, UnsignedByte, VarInt
};
use nbt_lib::datatypes::{TextComponent, NBT};
pub use spawn_entity::*;
/// Packet to spawn a experience orb
#[allow(missing_docs)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    /// The amount of ecperience that is contained in this orb
    pub count: Short,
}
mod entity_animation;
pub use entity_animation::*;
mod award_statistics;
pub use award_statistics::*;
/// An acknowledgement to signal the client, that the block change happened on the server and will
/// becarried out to the other clients
pub struct AcknowledgeBlockChange {
    /// The oid of the sequence
    pub sequence_id: VarInt,
}
/// Sets the destroy stage of a block at a specific block
///
/// The stage can be between 0 and 9 where 0 is a normal block and everything above 9 breaks the
/// block.
pub struct SetBlockDestroyStage {
    /// The id of the entity that destroys the block
    pub entity_id: VarInt,
    /// The location of the block
    pub location: Position,
    /// How far the block is destroyed
    ///
    /// # Note
    /// the range of this is 0..=9 everything above this should be considered like breaking the
    /// block
    pub destroy_stage: Byte,
}
/// Sets the block entity associated with the block at the given location.
#[allow(missing_docs)]
pub struct BlockEntityData {
    pub location: Position,
    pub block_entity_type: VarInt,
    /// Data to set
    ///
    /// # Note
    /// May be a TAG_END(0), is which case the block entity at the given location is removed
    /// if this has the value `None` a TAG_END(0) should be send
    pub nbt_data: Option<NBT>,
}
mod block_action;
pub use block_action::*;
/// Block changes within render distance
pub struct BlockUpdate {
    /// Block Coordinates
    pub location: Position,
    /// The new block state ID for the block
    pub block_id: VarInt,
}
mod boss_bar;
pub use boss_bar::*;
mod difficulty_change;
pub use difficulty_change::*;
/// Marks the end of a chunk batch
pub struct ChunkBatchFinished {
    /// Number of chunks
    pub batch_size: VarInt,
}
/// Marks the start of a Chunk Batch
pub struct ChunkBatchStart;
mod chunk_biomes;
pub use chunk_biomes::*;
/// Clears the clients title
#[allow(missing_docs)]
pub struct ClearTitles {
    pub reset: bool,
}
mod command_suggestion_response;
pub use command_suggestion_response::*;
/// This Packet sends a tree structure, what commands are possible to use on the server
pub struct Commands {
    /// The amount of nodes in the following array
    pub count: VarInt,
    /// An array of all used notes, that are referenced by indices
    pub nodes: Array<minecraft_assets::commands::CommandListNode>,
    /// The index of the root node, normally being 0
    pub root_index: VarInt,
}
/// Packet to forcibly close a window
pub struct CloseContainer {
    /// The id of the window
    pub window_id: UnsignedByte,
}
/// replaces the content of a container window
#[deprecated]
pub struct SetContainerContent {
    /// The window id
    pub window_id: UnsignedByte,
    /// Described here: <https://wiki.vg/Protocol#Click_Container>
    pub state_id: VarInt,
    /// The amount of of elements in the array
    pub count: VarInt,
    /// List of items in the window
    pub slot_data: Array<slot_lib::new::Slot>,
    /// The item carried by the mouse
    pub carried_item: slot_lib::new::Slot,
}
/// Updates a GUI window
#[deprecated]
pub struct SetContainerProperty;
/// Send to add/remove an item from a container slot
#[deprecated]
pub struct SetContainerSlot;
/// Requests a cookie
#[deprecated]
pub struct CookieRequest;
/// Set cooldown to all items of a given typV
#[deprecated]
pub struct SetCooldown;
/// Chat suggestion
#[deprecated]
pub struct ChatSuggestions;
/// Used for mods and plugins to communicate with the client
pub struct PluginMessage {
    /// The channel the data gets send to
    pub channel: Identifier,
    /// The data
    pub data: ByteArray,
}
/// Damage event
#[deprecated]
pub struct DamageEvent;
/// Debug Sample
#[deprecated]
pub struct DebugSample;
/// Delete Message
#[deprecated]
pub struct DeleteMessage;
/// Packet to disconnect the player
pub struct Disconnect {
    /// reason, why the player will be kicked
    pub reason: TextComponent,
}
/// Chat Message without signing information
#[deprecated]
pub struct DisguisedChatMessage;
/// Trigger to an entity event
#[deprecated]
pub struct EntityEvent;
/// Explosion
#[deprecated]
pub struct Explosion;
/// Tells the client to unload a chunk column
pub struct UnloadChunk {
    /// Block z coordinate divided by 16 and floored
    pub chunk_z: Int,
    /// Block x coordinate divided by 16 and floored
    pub chunk_x: Int,
}
mod game_event;
pub use game_event::*;
/// Open horse screen
#[deprecated]
pub struct OpenHorseScreen;
/// Hurt animation
#[deprecated]
pub struct HurtAnimation;
/// Used to let the client know, that the connection is still alive
pub struct KeepAlive {
    /// The id the client should respond with
    pub id: Long,
}
mod chunk_data_and_update_light;
pub use chunk_data_and_update_light::*;
/// World Event
#[deprecated]
pub struct WorldEvent;
/// Particle
#[deprecated]
pub struct Particle;
/// Update Light
#[deprecated]
pub struct UpdateLight;
mod login;
pub use login::*;
/// Map Data
#[deprecated]
pub struct MapData;
/// Merchant Offers
#[deprecated]
pub struct MerchantOffers;
/// Update Entity Position
pub struct UpdateEntityPosition {
    /// The id of the entity
    pub e_id: VarInt,
    /// The change in the x position
    pub delta_x: FixedPoint<Short, 4096>,
    /// The change in the y position
    pub delta_y: FixedPoint<Short, 4096>,
    /// The change in the z position
    pub delta_z: FixedPoint<Short, 4096>,
    /// Whether the entity is on the ground or not
    pub on_ground: Boolean,
}
/// Update Entity Position and Angle
pub struct UpdateEntityPositionAndAngle {
    /// The id of the entity
    pub e_id: VarInt,
    /// The change in the x position
    pub delta_x: FixedPoint<Short, 4096>,
    /// The change in the y position
    pub delta_y: FixedPoint<Short, 4096>,
    /// The change in the z position
    pub delta_z: FixedPoint<Short, 4096>,
    /// Whether the entity is on the ground or not
    pub on_ground: Boolean,
}
