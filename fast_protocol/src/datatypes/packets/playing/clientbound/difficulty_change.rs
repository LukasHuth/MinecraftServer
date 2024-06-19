use datatypes::{Enum, UnsignedByte};

/// Changes the difficulty display in the client
pub struct ChangeDifficulty {
    /// The difficulty to change to
    pub difficulty: Enum<change_difficulty::Difficulty, UnsignedByte>,
    /// whether the button should be locked or not
    pub locked: bool,
}
/// Additional datatypes for the `ChangeDifficulty` Packet
pub mod change_difficulty {
    use datatypes::ImportantEnumTrait;

    /// The difficulty of the world
    #[allow(missing_docs)]
    pub enum Difficulty {
        Peaceful = 0,
        Easy = 1,
        Normal = 2,
        Hard = 3,
    }
    impl ImportantEnumTrait for Difficulty {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Peaceful),
                1 => Ok(Self::Easy),
                2 => Ok(Self::Normal),
                3 => Ok(Self::Hard),
                _ => Err(binary_utils::Error::InvalidId)
            }
        }
    }
}
