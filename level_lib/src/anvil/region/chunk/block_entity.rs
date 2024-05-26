//! This module contaings data structs for block entity data
//!
//! # Sources
//! - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format#Block_entity_format)

mod chest_data;
use std::collections::HashMap;

pub use chest_data::*;

mod banner_data;
pub use banner_data::*;
use datatypes::Position;
use entmet_lib::entities::{entity::Bee, entity_types::EntityEnum};
use minecraft_assets::{effects::Effect, recipes::BlastFurnaceRecipe};
use nbt_lib::{
    convert_to_bool,
    traits::{AsNbtValue, FromNbtValue},
    unwrap_to_empty, unwrap_to_empty_if_exists, NbtValue,
};

/// A Struct for common struct data
#[derive(PartialEq, Debug, Default)]
pub struct BlockEntityData {
    /// The block id
    pub id: String,
    /// If true the block entity should not be placed
    pub keep_packed: bool,
    /// The X coordinate
    pub x: i32,
    /// The Y coordinate
    pub y: i32,
    /// The Z coordinate
    pub z: i32,
}

impl TryFrom<&HashMap<String, NbtValue>> for BlockEntityData {
    type Error = ();

    fn try_from(data: &HashMap<String, NbtValue>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: unwrap_to_empty!(data.get("id"), string),
            keep_packed: convert_to_bool!(unwrap_to_empty!(data.get("keepPacked"), i8)),
            x: unwrap_to_empty!(data.get("x"), i32),
            y: unwrap_to_empty!(data.get("y"), i32),
            z: unwrap_to_empty!(data.get("z"), i32),
        })
    }
}

/// A struct storing block entity data
///
/// These are blocks, that store more data than normal blocks. e.g. Chest. These blocks where
/// called "tile entities" until they where renamed in 1.18
///
/// # Sources
/// - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format#Block_entity_format)
///
/// # Example
/// ```
/// use level_lib::anvil::region::chunk::block_entity::{BlockEntity, ChestData, BlockEntityData};
/// let cd = ChestData::default();
/// let be = BlockEntity::Barrel{ chest_data: cd.clone() , block_entity_data:
/// BlockEntityData::default()};
/// let be_lock = match be {
///     BlockEntity::Barrel { chest_data, .. } => Some(chest_data),
///     _ => None,
/// };
/// assert!(matches!(be_lock, Some( ChestData { .. })));
/// let be_lock = be_lock.unwrap();
/// assert_eq!( be_lock.lock , cd.lock);
/// ```
#[deprecated(note = "This enum is still a work in progress and subject to change")]
#[derive(PartialEq, Debug)]
pub enum BlockEntity {
    /// Data block of an Block entity that only used the common tags used of all `BlockEntity`s
    ///
    /// # Used for
    /// - Bed
    /// - Bell
    Empty {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
    },
    /// Additional data of a `Banner`
    Banner {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// Optional custom name
        custom_name: Option<String>,
        /// The pattern data of the banner
        patterns: Vec<banner_data::BannerPattern>,
    },
    /// Aditional data of a `Barrel`
    Barrel {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// The chest data of the barrel
        chest_data: ChestData,
    },
    /// Additional data of a `Beacon`
    Beacon {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// Optional custom name
        custom_name: Option<String>,
        /// Optional lock
        lock: Option<String>,
        /// Level of the beacon
        ///
        /// # Note
        ///
        /// Always 0 if the beam is blocked
        levels: i32,
        /// The selected primary effect
        primary: Option<Effect>,
        /// The selected secondary effect
        secondary: Option<Effect>,
    },
    /// Additional data of a `Beehive`
    Beehive {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// The bees currently inside of the hive
        bees: Vec<BeeData>,
        /// Stores the location of a flower, so that the bees can go to it
        flower_pos: Position,
    },
    /// Additional data of a `BlastFurnace`
    BlastFurnace {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// The amount of ticks, until the current fuel runs out
        burn_time: u16,
        /// The amount of ticks, that the current item is already cooking for
        cook_time: u16,
        /// The amount of ticks it takes for the item to be smelted
        cook_time_total: u16,
        /// An optional custom name
        custom_name: Option<String>,
        /// List of items in the blast furnace
        ///
        /// # Note
        ///
        /// 1. Slot is the item beeing smelted
        /// 2. Slot is the item used as next fuel
        /// 3. Slot is the item in the result slot
        items: Vec<Item>,
        /// A list of all used recipes to calculate the stored xp
        recepies_used: Vec<BlastFurnaceRecipe>,
    },
    /// Additional data of a chest
    Chest {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// The chest data of the chest
        chest_data: ChestData,
    },
    /// Additional data of a brushable block
    BrushableBlock {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// The loot table of the bruchable block
        loot_table: String,
        /// the loot table seed of the brushable block
        loot_table_seed: i64,
    },
    /// Additional data of a brushable block
    #[allow(missing_docs)]
    MobSpawner {
        /// Common data to all Block entities
        block_entity_data: BlockEntityData,
        /// the delay until the next mob is spawned
        delay: i16,
        max_nearby_entities: Option<i16>,
        max_spawn_delay: Option<i16>,
        min_spawn_delay: Option<i16>,
        required_player_range: Option<i16>,
        spawn_count: Option<i16>,
        /// the nbt data that is copied onto the next mob that spawns
        spawn_data: Option<NbtValue>,
        /// List of possible entities to spawn
        ///
        /// # Note
        /// Not fully implemented
        spawn_potentials: Option<Vec<NbtValue>>,
        /// The radius around which the spawner attempts to place mobs randomly
        spawn_range: i16,
    },
}
impl FromNbtValue for BlockEntity {
    fn from_nbt_value(value: nbt_lib::NbtValue) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        match unwrap_to_empty!(data.get("id"), str) {
            "minecraft:chest" => Ok(Self::Chest {
                block_entity_data: BlockEntityData::try_from(&data)?,
                chest_data: ChestData::try_from(&data)?,
            }),
            "minecraft:barrel" => Ok(Self::Barrel {
                block_entity_data: BlockEntityData::try_from(&data)?,
                chest_data: ChestData::try_from(&data)?,
            }),
            "minecraft:brushable_block" => Ok(Self::BrushableBlock {
                block_entity_data: BlockEntityData::try_from(&data)?,
                loot_table: unwrap_to_empty!(data, "LootTable", string),
                loot_table_seed: unwrap_to_empty!(data, "LootTableSeed", i64),
            }),
            "minecraft:mob_spawner" => Ok(Self::MobSpawner {
                block_entity_data: BlockEntityData::try_from(&data)?,
                delay: unwrap_to_empty!(data, "Delay", i16),
                max_spawn_delay: unwrap_to_empty_if_exists!(data, "MaxSpawnDelay", i16),
                min_spawn_delay: unwrap_to_empty_if_exists!(data, "MinSpawnDelay", i16),
                max_nearby_entities: unwrap_to_empty_if_exists!(data, "MaxNearbyEntities", i16),
                required_player_range: unwrap_to_empty_if_exists!(data, "RequiredPlayerRange", i16),
                spawn_count: unwrap_to_empty_if_exists!(data, "SpawnCount", i16),
                spawn_data: None,
                spawn_potentials: None,
                spawn_range: unwrap_to_empty!(data, "SpawnRange", i16),
            }),
            _ => panic!("not_implemented: {:?}", data.get("id")),
        }
    }
}
impl AsNbtValue for BlockEntity {
    fn as_nbt_value(&self) -> Result<nbt_lib::NbtValue, ()> {
        todo!()
    }
}
/// A struct holding bee data
#[derive(PartialEq, Debug)]
pub struct BeeData {
    /// The data of the `Bee`
    pub bee: Bee,
    /// The minimum amount of ticks, that the bee stays in the hive
    pub min_occupation_ticks: i32,
    /// The amount of ticks, that the bee is in the hive
    pub ticks_in_hive: i32,
}

impl BlockEntity {
    /// Returns `true` if the block entity is [`Banner`].
    ///
    /// [`Banner`]: BlockEntity::Banner
    #[must_use]
    pub fn is_banner(&self) -> bool {
        matches!(self, Self::Banner { .. })
    }

    /// Returns `true` if the block entry stores [`ChestData`]
    ///
    /// [`ChestData`]: ChestData
    #[must_use]
    pub fn has_chest_data(&self) -> bool {
        match self {
            Self::Empty { .. } => false,
            Self::Banner { .. } => false,
            Self::Beacon { .. } => false,
            Self::Barrel { .. } => true,
            Self::Beehive { .. } => false,
            Self::BlastFurnace { .. } => false,
            Self::Chest { .. } => true,
            Self::BrushableBlock { .. } => false,
            Self::MobSpawner { .. } => false,
        }
    }

    /// Returns `true` if the block entity is [`Barrel`].
    ///
    /// [`Barrel`]: BlockEntity::Barrel
    #[must_use]
    pub fn is_barrel(&self) -> bool {
        matches!(self, Self::Barrel { .. })
    }

    /// Returns `true` if the block entity is [`Beacon`].
    ///
    /// [`Beacon`]: BlockEntity::Beacon
    #[must_use]
    pub fn is_beacon(&self) -> bool {
        matches!(self, Self::Beacon { .. })
    }

    /// Returns `true` if the block entity is [`Beehive`].
    ///
    /// [`Beehive`]: BlockEntity::Beehive
    #[must_use]
    pub fn is_beehive(&self) -> bool {
        matches!(self, Self::Beehive { .. })
    }

    /// Returns `true` if the block entity is [`Empty`].
    ///
    /// [`Empty`]: BlockEntity::Empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    /// Returns `true` if the block entity is [`BrushableBlock`].
    ///
    /// [`BrushableBlock`]: BlockEntity::BrushableBlock
    #[must_use]
    pub fn is_brushable_block(&self) -> bool {
        matches!(self, Self::BrushableBlock { .. })
    }

    /// Returns `true` if the block entity is [`Chest`].
    ///
    /// [`Chest`]: BlockEntity::Chest
    #[must_use]
    pub fn is_chest(&self) -> bool {
        matches!(self, Self::Chest { .. })
    }
}
