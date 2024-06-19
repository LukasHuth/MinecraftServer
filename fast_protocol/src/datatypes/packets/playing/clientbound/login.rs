use binary_utils::DataWriter;
use datatypes::{
    Array, Boolean, Byte, Identifier, ImportantFunctions as _, Int, Long, Position, UnsignedByte,
    VarInt,
};

/// This packet gets send to the player when another player logs onto the server
#[allow(missing_docs)]
pub struct Login {
    pub e_id: Int,
    pub is_hardcore: Boolean,
    pub dimension_count: VarInt,
    pub dimension_names: Array<Identifier>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: Boolean,
    pub enable_respawn_screen: Boolean,
    pub limit_crafting: Boolean,
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: Long,
    pub gamemode: login_extra::Gamemode,
    /// # Note
    /// if None, it gets send as a -1
    pub previous_gamemode: Option<login_extra::Gamemode>,
    pub is_debug: Boolean,
    pub is_flat: Boolean,
    pub death_data: Option<login_extra::DeathData>,
    pub portal_cooldown: VarInt,
    pub enforces_secure_char: Boolean,
}
#[allow(missing_docs)]
pub mod login_extra {
    use datatypes::{Identifier, Position};

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
    pub struct DeathData {
        pub dimension: Identifier,
        pub location: Position,
    }
}
