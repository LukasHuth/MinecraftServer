use datatypes::{BitMask, Boolean, Byte, Enum, Identifier, Long, UnsignedByte, VarInt};

/// Respawn The player, after death, credits or a portal
#[allow(missing_docs)]
pub struct Respawn {
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: Long,
    pub gamemode: Enum<respawn_extra::Gamemode, UnsignedByte>,
    /// `None` is -1
    pub previous_gamemode: Option<Enum<respawn_extra::Gamemode, Byte>>,
    pub is_debug: Boolean,
    pub is_flat: Boolean,
    pub death_info: Option<respawn_extra::DeathData>,
    pub protal_cooldown: VarInt,
    pub data_kept: BitMask<Byte, respawn_extra::DataKept>,
}
#[allow(missing_docs)]
pub mod respawn_extra {
    use datatypes::{Identifier, ImportantEnumTrait, Position, ToBitPos};

    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum Gamemode {
        Survival = 0,
        Creative = 1,
        Adventure = 2,
        Spectator = 3,
    }
    impl From<Gamemode> for u8 {
        #[inline]
        fn from(value: Gamemode) -> Self {
            match value {
                Gamemode::Survival => 0,
                Gamemode::Creative => 1,
                Gamemode::Adventure => 2,
                Gamemode::Spectator => 3,
            }
        }
    }
    impl From<Gamemode> for i8 {
        #[inline]
        fn from(value: Gamemode) -> Self {
            u8::from(value) as i8
        }
    }
    impl ImportantEnumTrait for Gamemode {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Survival),
                1 => Ok(Self::Creative),
                2 => Ok(Self::Adventure),
                3 => Ok(Self::Spectator),
                4..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
    pub struct DeathData {
        pub dimension: Identifier,
        pub location: Position,
    }
    pub enum DataKept {
        KeepAttributes,
        KeepMetadata,
    }
    impl ToBitPos for DataKept {
        fn to_bit_pos(&self) -> u64 {
            match self {
                Self::KeepAttributes => 1,
                Self::KeepMetadata => 2,
            }
        }

        fn iterator<'a>() -> std::slice::Iter<'a, Self> where Self: 'a + Sized {
            static LIST: [DataKept;2] = [DataKept::KeepAttributes, DataKept::KeepMetadata];
            LIST.iter()
        }
    }
}
