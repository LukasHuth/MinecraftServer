use datatypes::{BitMask, Byte, Double, Float, VarInt};

/// Teleports the client, e.g. during login, when using an ender pearl, in response to invalid move packets, etc.
#[allow(missing_docs)]
pub struct SynchronizePlayerPosition {
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub relative_values: BitMask<Byte, synchronize_player_position_extra::Fields>,
    pub teleport_id: VarInt,
}
#[allow(missing_docs)]
pub mod synchronize_player_position_extra {
    use datatypes::ToBitPos;

    pub enum Fields {
        X,
        Y,
        Z,
        Pitch,
        Yaw,
    }
    impl ToBitPos for Fields {
        fn to_bit_pos(&self) -> u64 {
            match self {
                Self::X => 1,
                Self::Y => 2,
                Self::Z => 3,
                Self::Pitch => 4,
                Self::Yaw => 5,
            }
        }

        fn iterator<'a>() -> std::slice::Iter<'a, Self> where Self: 'a + Sized {
            static LIST: [Fields; 5] = [Fields::X, Fields::Y, Fields::Z, Fields::Pitch, Fields::Yaw];
            LIST.iter()
        }
    }
}
