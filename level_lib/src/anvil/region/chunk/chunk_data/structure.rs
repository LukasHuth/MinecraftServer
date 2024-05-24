//! Module for structure data inside of a chunk

use std::{collections::HashMap, ops::Deref};

use nbt_lib::NbtValue;

use nbt_lib::unwrap_to_empty;

use self::{end_city_element::EndCityElement, jungle_temple::JungleTempleElement};

pub enum Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
impl Orientation {
    pub fn from(value: i32) -> Self {
        match value {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            i32::MIN..=-1 | 4..=i32::MAX => Self::North,
        }
    }
}

/// Module of an end city element
pub mod end_city_element;
/// Module for a jungle temple
pub mod jungle_temple;
/// Module for a strunghold
pub mod stronghold;
/// Module for an igloo
pub mod igloo;
/// Module for a nether fortress
pub mod nether_fortress;
/// Module for a mineshaft
pub mod mineshaft;
/// Module for an ocean monument
pub mod ocean_monument;
/// Module for an ocean ruin
pub mod ocean_ruin;
/// Module for a shipwrack
pub mod ship_wrack;
/// Module for a swarmp hut
pub mod swarmp_hut;
/// Module for a burried trasure
pub mod burried_treasure;
/// Module for a woodland mansion
pub mod woodland_mansion;
/// Module for a dessert temple
pub mod dessert_temple;
// Bastion ???


/// An enum of the differenct types of structure data
///
/// # Sources
/// - [minecraft german fandom](https://minecraft.fandom.com/de/wiki/Bauwerksdaten#Basisdaten)
pub enum StructureData {
    /// Structure data of an end city
    EndCity {
        /// The basic data of a structure
        basic_data: BasicStructureData,
        /// All elements of the end city
        children: Vec<EndCityElement>,
    },
    /// Structure data of a `JungleTemple`
    JungleTemple {
        /// The basic data of a structure
        basic_data: BasicStructureData,
        children: Vec<JungleTempleElement>
    },
    /// Structure data of a `Stronghold`
    Stronghold {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `Igloo`
    Igloo {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `NetherFortress`
    NetherFortress {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `Mineshaft`
    Mineshaft {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `OceanMonument`
    OceanMonument {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `OceanRuin`
    OceanRuin {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `ShipWrack`
    ShipWrack {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `SwarmpHut`
    SwarmpHut {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `BurriedTreasure`
    BurriedTreasure {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `WoodlandMansion`
    WoodlandMansion {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `DessertTemple`
    DessertTemple {
        /// The basic data of a structure
        basic_data: BasicStructureData,
    },
    /// Structure data of a `Bastion`
    Bastion {}
}
impl StructureData {
    pub fn from_nbt(name: String, values: HashMap<String, nbt_lib::NbtValue>) -> Result<Self, ()> where Self: Sized {
        let basic_data = BasicStructureData::from(values)?;
        todo!();
        /*
        match name.as_str() {
        "Jungle_Pyramid" => Ok(Self::JungleTemple { basic_data , children: convert_list_to!(values.get("Children"), JungleTempleElement) })
        }
        */
    }
}
impl Deref for StructureData {
    type Target = BasicStructureData;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::EndCity { basic_data, .. } => basic_data,
            Self::JungleTemple { basic_data, .. } => basic_data,
            Self::Stronghold { basic_data } => basic_data,
            Self::Igloo { basic_data } => basic_data,
            Self::NetherFortress { basic_data } => basic_data,
            Self::Mineshaft { basic_data } => basic_data,
            Self::OceanMonument { basic_data } => basic_data,
            Self::OceanRuin { basic_data } => basic_data,
            Self::ShipWrack { basic_data } => basic_data,
            Self::SwarmpHut { basic_data } => basic_data,
            Self::BurriedTreasure { basic_data } => basic_data,
            Self::WoodlandMansion { basic_data } => basic_data,
            Self::DessertTemple { basic_data } => basic_data,
            Self::Bastion { .. } => unimplemented!(),
        }
    }
}

/// A struct for structure data
pub struct BasicStructureData {
    /// The six coordinated of the bounding box lower x, y, z and upper x, y, z
    pub bounding_box: [i32;6],
    /// The biome id the structure is in
    pub biome: String,
    /// The chunk x coordinate of this structure
    pub chunk_x: i32,
    /// The chunk z coordinate of this structure
    pub chunk_z: i32,
    /// The id of the structure
    pub id: String,
}
impl BasicStructureData {
    pub fn from(values: HashMap<String, NbtValue>) -> Result<Self, ()> {
        Ok(Self {
            biome: unwrap_to_empty!(values.get("biome"), string),
            bounding_box: unwrap_to_empty!(values.get("BB"), i32_array).try_into().unwrap(),
            chunk_x: unwrap_to_empty!(values.get("ChunkX"), i32),
            chunk_z: unwrap_to_empty!(values.get("ChunkZ"), i32),
            id: unwrap_to_empty!(values.get("id"), string),
        })
    }
}
/// A struct for a piece of a structure
///
/// [Source](https://minecraft.fandom.com/wiki/Chunk_format#NBT_structure)
///
/// # ToDO
///
/// - Change this to an enum for the different structures:
///   - [Source](https://minecraft.fandom.com/de/wiki/Bauwerksdaten)
#[deprecated]
pub struct StructurePiece {
    /// The six coordinated of the bounding box lower x, y, z and upper x, y, z
    pub bounding_box: [i32;6],
    /// The ocean temperature a ocean ruin is in
    pub biome_type: Option<BiomeType>,
    /// - Fortress "NeSCLT" and "NeSCRT":
    ///   - Whether this fortress piece should contain a chest but hasn't had one generated yet.
    /// - Stronghold "SHCC":
    ///   - Whether this chest in this stronghold piece was placed.
    /// - Village "ViS":
    ///   - Whether the blacksmith chest has been generated.
    pub chest: Option<bool>,
}
/// An enum for Biome types (Warm or cold)
pub enum BiomeType {
    /// Inside a warm biome
    Warm,
    /// Inside a cold biome
    Cold,
}
/// A list of all structure pieces
///
/// # Source
///
/// - [German minecraft fandom](https://minecraft.fandom.com/de/wiki/Bauwerksdaten)
pub enum StructurePieceId {
    /// Start of the stronghold
    ///
    /// # Note
    ///
    /// This type has a NBT boolean if it is at the start, which is kinda unnessecarry, because
    /// that is called `SHStart`
    SHStart,
    /// Stronghold Flexible corridor
    SHFC,
    /// Stronghold left turn
    SHLT,
    /// Stronghold straight corridor
    ///
    /// # Note
    ///
    /// This section can have a left connection and/or a right connection, they are noted by an
    /// extra NBT boolean called `Left` and `Right` which  contain `true` if the connection is
    /// opened
    SHS,
    /// Stringhold 5 corridors piece
    ///
    /// # Note
    ///
    /// This section can have four optional connections, they are noted by extra NBT booleans named
    /// `leftHigh`, `leftLow`, `rightHigh` and `rightLow`
    SH5C,
    /// Stronghold "round" staircase down
    ///
    /// # Note
    ///
    /// This type has a NBT boolean if it is at the start, which is kinda unnessecarry, because
    /// that is called `SHStart`
    SHSD,
    /// Stronghold straight stairs down
    SHSSD,
    /// Stronghold Chest corridor
    ///
    /// # Note
    ///
    /// This has an NBT boolean whether the chest is already generated or not
    SHCC,
    /// Stronghold prison hall
    SHPH,
    /// Stronghold representative chamber
    ///
    /// # Note
    ///
    /// This includes an NBT int that has one of the options of [`SHRepresentativeType`]
    SHRC,
    /// Stronghold library
    ///
    /// # Note
    ///
    /// This adds a NBT boolean `Tall` that determines whether the library has one or two floors
    SHLi,
    /// Stronghold protal room
    ///
    /// # Note
    ///
    /// This add a NBT boolean `Mob` that determines whether the silverfish spawner was generated
    /// or not
    SHPR,
    /// An end city piece
    ///
    /// # Note
    ///
    /// All end city pieces have this identifier, they are differenciated with the `Template` NBT
    /// Tag
    ECP,
}

/// A list of all decorations possible in a [`SHRC`]
///
/// [`SHRC`]: StructurePieceId::SHRC
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum SHRepresentativeType {
    TorchPillar = 0,
    Fountain = 1,
    Chest = 2,
    Empty = 3,
}
