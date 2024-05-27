//! A module for all clientbound packets that are beeing send while playing

/// The delimiter for a bundle of packets.
///
/// It ensures that every packet after it until getting the
/// of is only stored and will be executed at the same time, it is for example used to spawn every
/// entity of the same type on the same tick
pub struct BundleDelimiter;
mod spawn_entity;
use datatypes::{Double, Short, VarInt};
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
