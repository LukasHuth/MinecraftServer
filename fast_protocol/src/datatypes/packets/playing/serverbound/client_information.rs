use datatypes::{BitMask, Boolean, Byte, Enum, UnsignedByte, VarInt};

/// Client Information
pub struct ClientInformation {
    /// The language that the client uses
    pub locale: datatypes::String,
    /// The view distance of the client
    pub view_distance: Byte,
    /// The cchat mode of the client
    pub chat_mode: Enum<client_information_extra::ChatMode, VarInt>,
    /// Whether the client uses colored chat or not
    pub chat_colors: Boolean,
    /// The displayed skin parts
    pub displayed_skin_parts: BitMask<UnsignedByte, client_information_extra::DisplayedSkinParts>,
    /// The main hand
    pub main_hand: Enum<client_information_extra::MainHand, VarInt>,
    ///
    pub enable_text_filtering: Boolean,
    ///
    pub allow_server_listings: Boolean,
}
#[allow(missing_docs)]
pub mod client_information_extra {
    use datatypes::{ImportantEnumTrait, ToBitPos};

    pub enum ChatMode {
        Enabled,
        CommandsOnly,
        Hidden,
    }
    impl ImportantEnumTrait for ChatMode {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Enabled),
                1 => Ok(Self::CommandsOnly),
                2 => Ok(Self::Hidden),
                3..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
    pub enum DisplayedSkinParts {
        Cape,
        Jacket,
        LeftSleve,
        RightSleve,
        LeftPantsLeg,
        RightPantsLeg,
        Hat,
    }
    impl ToBitPos for DisplayedSkinParts {
        fn to_bit_pos(&self) -> u64 {
            match self {
                Self::Cape => 0,
                Self::Jacket => 1,
                Self::LeftSleve => 2,
                Self::RightSleve => 3,
                Self::LeftPantsLeg => 4,
                Self::RightPantsLeg => 5,
                Self::Hat => 6,
            }
        }

        fn iterator<'a>() -> std::slice::Iter<'a, Self> where Self: 'a + Sized {
            use DisplayedSkinParts::*;
            static LIST: [DisplayedSkinParts; 7] = [Cape, Jacket, LeftSleve, RightSleve, LeftPantsLeg, RightPantsLeg, Hat];
            LIST.iter()
        }
    }
    pub enum MainHand {
        Left,
        Right,
    }
    impl ImportantEnumTrait for MainHand {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Left),
                1 => Ok(Self::Right),
                2..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
}
