use nbt_lib::NbtValue;

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
    pub loottable: Option<LootTable>,
    /// Optional seed for the loot table
    ///
    /// # Note
    /// 
    /// After generating the items, this field will be emptied
    pub loottable_seed: i64,
}
