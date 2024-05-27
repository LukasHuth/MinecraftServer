use datatypes::{Angle, Double, Int, Short, VarInt, UUID};

/// Data for an itemframe
#[allow(missing_docs)]
pub enum ItemFrameData {
    Down,
    Up,
    North,
    South,
    West,
    East,
}
/// The data of a painting
#[allow(missing_docs)]
pub enum PaintingData {
    North,
    South,
    West,
    East
}
/// The data of a falling block
#[allow(missing_docs)]
pub enum FallingBlockData {
}
/// The spawn entity object data, only used on some entities
#[allow(missing_docs)]
pub enum SpawnEntityObjectData {
    ItemFrame {
        data: ItemFrameData
    },
    Painting {
        data: PaintingData
    },
    FallingBlock {
        block_state_id: Int,
    },
    FishingHook {
        /// Entity id of the owner
        owner: Int,
    },
    Protectile {
        owner_id: Int,
    },
    /// Warden data
    ///
    /// # Source
    /// <https://wiki.vg/Object_Data#Warden>
    Warden {
        pose: Int,
    },
    None
}
/// The packet for spawning an entity
#[allow(missing_docs)]
pub struct SpawnEntity {
    pub entity_id: VarInt,
    /// The uuid of the entity, if has to be a valid uuid with skin blob in online mode, but not
    /// necessary, since this implementation is currently offline
    ///
    /// # Note
    ///
    /// UUIDv3 is used for players and UUIDv2 for NPCs to avoid collision
    /// In an example UUID, xxxxxxxx-xxxx-Yxxx-xxxx-xxxxxxxxxxxx, the UUID version is specified by
    /// Y. So, for UUID v3, Y will always be 3, and for UUID v2, Y will always be 2.
    ///
    /// # Source
    /// <https://wiki.vg/Protocol#Spawn_Entity>
    pub entity_uuid: UUID,
    pub type_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    pub data: SpawnEntityObjectData,
    pub velocity_x: Short,
    pub velocity_y: Short,
    pub velocity_z: Short,
}
