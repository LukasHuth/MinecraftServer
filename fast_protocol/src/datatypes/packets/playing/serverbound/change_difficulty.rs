use datatypes::{Byte, Enum};

/// Change Difficulty
pub struct ChangeDifficulty {
    /// The new difficulty requested by the client
    pub new_difficulty: Enum<change_difficulty_extra::Difficulty, Byte>
}
#[allow(missing_docs)]
pub mod change_difficulty_extra {
    use datatypes::ImportantEnumTrait;

    pub enum Difficulty {
        Peaceful,
        Easy,
        Normal,
        Hard,
    }
    impl ImportantEnumTrait for Difficulty {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Peaceful),
                1 => Ok(Self::Easy),
                2 => Ok(Self::Normal),
                3 => Ok(Self::Hard),
                4..=u64::MAX => Err(binary_utils::Error::InvalidId)
            }
        }
    }
}
