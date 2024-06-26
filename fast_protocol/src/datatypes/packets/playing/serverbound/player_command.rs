use datatypes::{Enum, VarInt};

/// Player Command
#[allow(missing_docs)]
pub struct PlayerCommand {
    pub e_id: VarInt,
    pub action_id: Enum<player_command_extra::Action, VarInt>,
    pub jump_boost: VarInt,
}
#[allow(missing_docs)]
pub mod player_command_extra {
    use datatypes::ImportantEnumTrait;

    pub enum Action {
        StartSneaking,
        StopSneaking,
        LeaveBed,
        StartSprinting,
        StopSprinting,
        StartJumpWithHorse,
        StopJumpWithHorse,
        OpenVehicleInventory,
        StartFlyingWithElytra,
    }
    impl ImportantEnumTrait for Action {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::StartSneaking),
                1 => Ok(Self::StopSneaking),
                2 => Ok(Self::LeaveBed),
                3 => Ok(Self::StartSprinting),
                4 => Ok(Self::StopSprinting),
                5 => Ok(Self::StartJumpWithHorse),
                6 => Ok(Self::StopJumpWithHorse),
                7 => Ok(Self::OpenVehicleInventory),
                8 => Ok(Self::StartFlyingWithElytra),
                9..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
}
