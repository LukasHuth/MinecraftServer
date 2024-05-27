use datatypes::VarInt;

/// Animation of an entity
#[allow(missing_docs)]
#[repr(u8)]
pub enum Animation {
    SwingMainArm = 0,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}
/// An packet to change the animation of an entity
#[allow(missing_docs)]
pub struct EntityAnimation {
    pub enitity_id: VarInt,
    /// Send as a `UnsignedByte`
    pub animation: Animation,
}
