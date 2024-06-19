use std::collections::HashMap;

use nbt_lib::{convert_list_to, traits::FromNbtValue, unwrap_to_empty, unwrap_to_empty_if_exists, NbtValue};

/// A structure for chest item data
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Item {
    /// The amount  of the items
    pub count: u8,
    /// The slot that the item is in
    pub slot: u8,
    /// The id of the Item/Block
    pub id: String,
    /// Aditional data
    pub tag: Option<NbtValue>,
}
impl FromNbtValue for Item {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        Ok(Self {
            count: unwrap_to_empty!(data, "Count", i8) as u8,
            slot: unwrap_to_empty!(data, "Slot", i8) as u8,
            id: unwrap_to_empty!(data, "id", string),
            tag: if data.contains_key("tag") { data.get("tag").map(|v|v.clone()) } else { None },
        })
    }
}
/// A list of all vanilla loot tables
#[derive(Default, Clone, Debug, PartialEq)]
pub enum VanillaLootTable {
    /// A placeholder
    #[default] Stronghold,
}
/// An enum to differentiate between vanilla and custom loot tables
#[derive(Clone, Debug, PartialEq)]
pub enum LootTable {
    /// The loot table is a vanilla one
    Vanilla(VanillaLootTable),
    /// TODO: implement this
    Custom,
}
impl Default for LootTable {
    fn default() -> Self {
        Self::Vanilla(VanillaLootTable::default())
    }
}
/// A Struct for every `BlockEntity` that has an inventory
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChestData {
    /// Optional custom name
    pub custom_name: Option<String>,
    /// A list of all items of the inventory
    pub items: Vec<Item>,
    /// When not blank, the container cannot be opened unless the player is holding an item
    /// with an identical name
    pub lock: Option<String>,
    /// Optional loottable, that defines, what can generate in the chest
    ///
    /// # Note
    /// 
    /// After generating the items, this field will be emptied
    ///
    /// # Info
    /// this could be changed to [`LootTable`]
    ///
    /// [`LootTable`]: `crate::anvil::region::chunk::block_entity::chest_data::LootTable`
    pub loottable: Option<String>,
    /// Optional seed for the loot table
    ///
    /// # Note
    /// 
    /// After generating the items, this field will be emptied
    pub loottable_seed: Option<i64>,
}
impl TryFrom<&HashMap<String, NbtValue>> for ChestData {
    type Error = ();

    fn try_from(value: &HashMap<String, NbtValue>) -> Result<Self, Self::Error> {
        Ok(Self {
            custom_name: unwrap_to_empty_if_exists!(value, "CustomName", string),
            lock: unwrap_to_empty_if_exists!(value, "Lock", string),
            loottable: unwrap_to_empty_if_exists!(value, "LootTable", string),
            loottable_seed: unwrap_to_empty_if_exists!(value, "LootTableSeed", i64),
            items: convert_list_to!(value, "Items", Item)
        })
    }
}
