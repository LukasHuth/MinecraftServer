use datatypes::{Enum, VarInt};
use nbt_lib::datatypes::TextComponent;

/// Open screen opens an inventory
pub struct OpenScreen {
    /// Window id
    pub w_id: VarInt,
    /// Window type
    pub w_type: Enum<open_screen_extra::InventoryType, VarInt>,
    /// Window title
    pub w_title: TextComponent,
}
mod open_screen_extra {
    use datatypes::ImportantEnumTrait;

    /// Source: <https://wiki.vg/Inventory>
    pub enum InventoryType {
        Generic9x1,
        Generic9x2,
        Generic9x3,
        Generic9x4,
        Generic9x5,
        Generic9x6,
        Generic3x3,
        Crafter3x3,
        Anvil,
        Beacon,
        BlastFurnace,
        BrewingStand,
        Crafting,
        Enchantment,
        Furnace,
        Grindstone,
        Hopper,
        Lectern,
        Loom,
        Merchant,
        ShulkerBox,
        Smithing,
        Smoker,
        Cartography,
        Stonecutter,
    }
    impl ImportantEnumTrait for InventoryType {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::Generic9x1),
                1 => Ok(Self::Generic9x2),
                2 => Ok(Self::Generic9x3),
                3 => Ok(Self::Generic9x4),
                4 => Ok(Self::Generic9x5),
                5 => Ok(Self::Generic9x6),
                6 => Ok(Self::Generic3x3),
                7 => Ok(Self::Crafter3x3),
                8 => Ok(Self::Anvil),
                9 => Ok(Self::Beacon),
                10 => Ok(Self::BlastFurnace),
                11 => Ok(Self::BrewingStand),
                12 => Ok(Self::Crafting),
                13 => Ok(Self::Enchantment),
                14 => Ok(Self::Furnace),
                15 => Ok(Self::Grindstone),
                16 => Ok(Self::Hopper),
                17 => Ok(Self::Lectern),
                18 => Ok(Self::Loom),
                19 => Ok(Self::Merchant),
                20 => Ok(Self::ShulkerBox),
                21 => Ok(Self::Smithing),
                22 => Ok(Self::Smoker),
                23 => Ok(Self::Cartography),
                24 => Ok(Self::Stonecutter),
                25..=u64::MAX => Err(binary_utils::Error::InvalidId),
            }
        }
    }
}
