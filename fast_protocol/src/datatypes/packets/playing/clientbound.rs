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
use datatypes::{Byte, Double, Position, Short, VarInt};
use nbt_lib::datatypes::NBT;
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
