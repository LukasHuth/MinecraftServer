use datatypes::ImportantEnumTrait;
use super::*;

impl ImportantEnumTrait for ChatMode {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            3..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl ImportantEnumTrait for MainHand {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            2..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl ImportantEnumTrait for ResourcePackResponseEnum {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(Self::SuccessfullyDownloaded),
            1 => Ok(Self::Declined),
            2 => Ok(Self::FailedToDownload),
            3 => Ok(Self::Accepted),
            4 => Ok(Self::Downloaded),
            5 => Ok(Self::InvalidUrl),
            6 => Ok(Self::FailedToReload),
            7 => Ok(Self::Discarded),
            8..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
