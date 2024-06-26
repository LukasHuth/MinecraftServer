use datatypes::{Enum, VarInt};

/// THe client status
pub struct ClientStatus {
    /// The status
    pub action: Enum<client_status_extra::Action, VarInt>,
}
#[allow(missing_docs)]
pub mod client_status_extra {
    use datatypes::ImportantEnumTrait;

    pub enum Action {
        PerformRespawn,
        RequestStats,
    }
    impl ImportantEnumTrait for Action {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::PerformRespawn),
                1 => Ok(Self::RequestStats),
                2..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
}
