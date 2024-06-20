use datatypes::{BitMask, Byte, Float};

/// Player abilities
pub struct PlayerAbilities {
    /// Flags
    pub flags: BitMask<Byte, player_abilities_extra::PlayerAbilitiesFlags>,
    /// Default: 0.05
    pub flying_speed: Float,
    /// Default: 0.1
    pub field_of_view_modifier: Float,
}
#[allow(missing_docs)]
pub mod player_abilities_extra {
    use datatypes::ToBitPos;

    pub enum PlayerAbilitiesFlags {
        Invulnerable,
        Flying,
        AllowFlying,
        CreativeMode,
    }
    impl ToBitPos for PlayerAbilitiesFlags {
        fn to_bit_pos(&self) -> u64 {
            match self {
                Self::Invulnerable => 0,
                Self::Flying => 1,
                Self::AllowFlying => 2,
                Self::CreativeMode => 3,
            }
        }

        fn iterator<'a>() -> std::slice::Iter<'a, Self>
        where
            Self: 'a + Sized,
        {
            static LIST: [PlayerAbilitiesFlags; 4] = [
                PlayerAbilitiesFlags::Invulnerable,
                PlayerAbilitiesFlags::Flying,
                PlayerAbilitiesFlags::AllowFlying,
                PlayerAbilitiesFlags::CreativeMode,
            ];
            LIST.iter()
        }
    }
}
