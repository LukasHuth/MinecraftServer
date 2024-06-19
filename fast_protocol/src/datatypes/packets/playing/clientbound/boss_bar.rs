use datatypes::UUID;

/// Every datatype needed for the `BossBar` Packet
pub mod boss_bar_additionals {
    use datatypes::{Enum, Float, ImportantEnumTrait, VarInt};
    use entmet_lib::datatypes::Mask;
    use nbt_lib::datatypes::TextComponent;

    /// The action of the Boss bar
    #[allow(missing_docs)]
    pub enum Action {
        Add {
            title: TextComponent,
            health: Float,
            color: Enum<Color, VarInt>,
            division: Enum<Division, VarInt>,
            flags: Mask<FlagData>,
        },
        Remove,
        UpdateHealth {
            health: Float,
        },
        UpdateTitle {
            title: TextComponent,
        },
        UpdateStyle {
            color: Enum<Color, VarInt>,
            dividers: Enum<Division, VarInt>,
        },
        UpdateFlags {
            flags: Mask<FlagData>,
        },
    }
    /// A Bossbar color
    #[allow(missing_docs)]
    #[repr(u8)]
    pub enum Color {
        Pink = 0,
        Blue = 1,
        Red = 2,
        Green = 3,
        Yellow = 4,
        Purple = 5,
        White = 6,
    }
    impl ImportantEnumTrait for Color {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Pink),
                1 => Ok(Self::Blue),
                2 => Ok(Self::Red),
                3 => Ok(Self::Green),
                4 => Ok(Self::Yellow),
                5 => Ok(Self::Purple),
                6 => Ok(Self::White),
                _ => Err(binary_utils::Error::InvalidId),
            }
        }
    }
    /// A Bossbar division
    #[allow(missing_docs)]
    #[repr(u8)]
    pub enum Division {
        NoDivision = 0,
        SixNotches = 1,
        TenNotches = 2,
        TwelveNotches = 3,
        TwentyNotches = 4,
    }
    impl ImportantEnumTrait for Division {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::NoDivision),
                1 => Ok(Self::SixNotches),
                2 => Ok(Self::TenNotches),
                3 => Ok(Self::TwelveNotches),
                4 => Ok(Self::TwentyNotches),
                _ => Err(binary_utils::Error::InvalidId),
            }
        }
    }
    #[allow(missing_docs)]
    #[repr(u8)]
    pub enum FlagData {
        DarkenSky = 0x1,
        DragonBar = 0x2,
        CreateFog = 0x4
    }
}
/// A Packet to send the Boss Bar
pub struct BossBar {
    /// The uuid of the boss bar
    pub uuid: UUID,
    /// The action that should be executed on it
    pub action: boss_bar_additionals::Action,
}
