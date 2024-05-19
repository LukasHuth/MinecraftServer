//! A module for chunk data
use std::{collections::HashMap, ops::{Deref, DerefMut}, str::FromStr};

use nbt_lib::{traits::{AsNbtValue, FromNbtValue, IntoNbt}, NbtValue};
use serde::{Deserialize, Serialize};

use crate::{anvil::region::chunk::section::ChunkSection, convert_list_to, create_compound_map, unwrap_to_empty, unwrap_to_empty_if_exists};

use super::block_entity::BlockEntity;

/// Enum of all Generation states
#[derive(Serialize, Deserialize)]
pub enum GenerationStatus {
    /// Chunk is not generated
    #[serde(rename = "empty")]
    Empty,
    /// The structures where started to being generated
    #[serde(rename = "structure_starts")]
    StructureStarts,
    /// IDK, hopefully i update this the moment i finished the world generation an know what step
    /// this is
    #[serde(rename = "structure_references")]
    StructureReferences,
    /// The biomes are started to being generated
    #[serde(rename = "biomes")]
    Biomes,
    /// The noise for the chunk is generated
    #[serde(rename = "noise")]
    Noise,
    /// The surface of the chunk is generated
    #[serde(rename = "surface")]
    Surface,
    /// The Caves of the chunk are generated? (guessing, because i didn't worked at the world
    /// generation at the point of writing this code)
    #[serde(rename = "cavers")]
    Carvers,
    /// Liquid generation ig? 
    #[serde(rename = "liquid_cavers")]
    LiquidCarvers,
    /// Generating features?
    #[serde(rename = "features")]
    Features,
    /// Generating light information
    #[serde(rename = "light")]
    Light,
    /// Generation spawn information
    #[serde(rename = "spawn")]
    Spawn,
    /// Generating the heightmaps of the chunk
    #[serde(rename = "heightmaps")]
    Heightmaps,
    /// The cunk is fully generated
    #[serde(rename = "minecraft:full")]
    Full,
}
impl ToString for GenerationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Full => String::from("minecraft:full"),
            Self::Empty => String::from("empty"),
            Self::StructureStarts => String::from("structure_starts"),
            Self::StructureReferences => String::from("structure_references"),
            Self::Biomes => String::from("biomes"),
            Self::Noise => String::from("noise"),
            Self::Surface => String::from("surface"),
            Self::Carvers => String::from("carvers"),
            Self::LiquidCarvers => String::from("liquid_carvers"),
            Self::Features => String::from("features"),
            Self::Light => String::from("light"),
            Self::Spawn => String::from("spawn"),
            Self::Heightmaps => String::from("heightmaps"),
        }
    }
}
impl FromStr for GenerationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "minecraft:full" => Ok(Self::Full),
            "empty" => Ok(Self::Empty),
            "structure_starts" => Ok(Self::StructureStarts),
            "structure_references" => Ok(Self::StructureReferences),
            "biomes" => Ok(Self::Biomes),
            "noise" => Ok(Self::Noise),
            "surface" => Ok(Self::Surface),
            "carvers" => Ok(Self::Carvers),
            "liquid_carvers" => Ok(Self::LiquidCarvers),
            "features" => Ok(Self::Features),
            "light" => Ok(Self::Light),
            "spawn" => Ok(Self::Spawn),
            "heightmaps" => Ok(Self::Heightmaps),
            _ => {
                if s.starts_with("minecraft:") {
                    Self::from_str(&s[..10])
                } else {
                    Err(())
                }
            }
        }
    }
}
/// Definition of the layout of an heightmap
///
/// # Note
///
/// Every element(u64) stores 7 values each of them 9-bits long
pub type Heightmap = [u64;37];
impl HeightMapIdentifier for Heightmap {
    fn get_default() -> Self {
        [0;37]
    }
}
trait HeightMapIdentifier {
    fn get_default() -> Self;
}
/// A Struct holding all Heightmaps
pub struct Heightmaps {
    /// A Heightmap of the heigest motion blocking block in the section
    pub motion_blocking: HeightmapWrapper,
    /// A Heightmap of the heigest motion blocking block in the section, excluding leaves
    pub motion_blocking_no_leaves: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest ocean floor block in the section
    pub ocean_floor: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest ocean floor block in the section (use case of this is currently
    /// unknown to me)
    pub ocean_floor_wg: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest world surface block in the section
    pub world_surface: HeightmapWrapper,
    /// A Heightmap of the heigest world surface block in the section (use case of this is currently
    /// unknown to me)
    pub world_surface_wg: Option<HeightmapWrapper>,
}
impl FromNbtValue for Heightmaps {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        let motion_blocking = unwrap_to_empty!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).try_into().unwrap();
        let motion_blocking_no_leaves = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let ocean_floor = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let ocean_floor_wg = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let world_surface = unwrap_to_empty!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).try_into().unwrap();
        let world_surface_wg = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        Ok(Self {
            motion_blocking,
            motion_blocking_no_leaves,
            ocean_floor,
            ocean_floor_wg,
            world_surface,
            world_surface_wg,
        })
    }
}
impl Default for Heightmaps {
    fn default() -> Self {
        Self {
            motion_blocking: Default::default(),
            motion_blocking_no_leaves: Default::default(),
            ocean_floor: Default::default(),
            ocean_floor_wg: Default::default(),
            world_surface: Default::default(),
            world_surface_wg: Default::default(),
        }
    }
}
/// wrapper for heightmaps to be able to implement default and similar things
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HeightmapWrapper {
    data: Heightmap
}
impl TryFrom<Vec<u64>> for HeightmapWrapper {
    type Error = Vec<u64>;

    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        Ok(HeightmapWrapper { data: Heightmap::try_from(value)? })
    }
}
impl TryFrom<Vec<i64>> for HeightmapWrapper {
    type Error = Vec<u64>;

    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        Ok(HeightmapWrapper { data: Heightmap::try_from(value.into_iter().map(|v| v as u64).collect::<Vec<_>>())? })
    }
}
impl Deref for HeightmapWrapper {
    type Target = Heightmap;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for HeightmapWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl Default for HeightmapWrapper {
    fn default() -> Self {
        // Provide a default implementation for T here
        // This will depend on what a reasonable default for your type T is
        // For the sake of example, let's assume you have a function to create a default instance
        HeightmapWrapper{data: Heightmap::get_default()}
    }
}
impl AsNbtValue for Heightmaps {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        todo!()
    }
}
/// A struct holding all chunk data
///
/// # Source
/// - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format#NBT_structure)
///
/// # Info
/// Everything, taged as "Not confirmed for 1.18 format" is not implemented, but will be if it
/// turnes out to be neccessarry
pub struct ChunkData {
    /// The real x position of the chunk
    pub x_pos: i32,
    /// The read z position of the chunk
    pub z_pos: i32,
    /// The lowest y sector position in the chunk
    pub y_pos: i32,
    /// The generation status of the chunk
    pub status: GenerationStatus,
    /// The last tick that the chunk was updated
    pub last_update: i64,
    /// A list of all sections of the chunk
    pub sections: Vec<super::section::ChunkSection>,
    /// A list of all block entities in a chunk
    pub block_entities: Vec<BlockEntity>,
    // Info: CarvingMasks are only used for proto chunk and will probably be added to this the
    // moment i write the world generation and learn, how and what exactly it stores
    /// All heightmaps of the section
    pub heightmaps: Heightmaps,
    /// A list of "active" liquid tiles in the chunk
    pub fluid_ticks: Vec<TileTick>,
    /// A list of "active" blocks in the chunk
    pub block_ticks: Vec<TileTick>,
    /// This variable increases tick by the amount of players inside of it
    pub inhabited_time: i64,
    ///// A list of 24 (the amount of sections) lists. Each one of them holding data for blocks that
    ///// have to be updated, this is only used in proto chunks
    // pub post_processing: [Vec<ToBeTicked>; 24],
    /// A list of all structures and their data
    pub structure_data_list: StructureDataList,
}
impl ChunkData {
    const X_POS: &'static str = "xPos";
    const Y_POS: &'static str = "yPos";
    const Z_POS: &'static str = "zPos";
    const STATUS: &'static str = "Status";
    const LAST_UPDATE: &'static str = "LastUpdate";
    const SECTIONS: &'static str = "sections";
    const BLOCK_ENTITIES: &'static str = "block_entities";
    const HEIGHTMAPS: &'static str = "Heightmaps";
    const FLUID_TICKS: &'static str = "fluid_ticks";
    const BLOCK_TICKS: &'static str = "block_ticks";
    const INHABITED_TIME: &'static str = "InhabitedTime";
    const STRUCTURES: &'static str = "structures";
    /// creates a new instance
    pub fn new(heightmap: [[u16; 16]; 16], filler: &str, x: i64, z: i64, current_tick: i64) -> Self {
        let mut s = Self {
            x_pos: x as i32,
            y_pos: -4,
            z_pos: z as i32,
            status: GenerationStatus::Empty,
            last_update: current_tick,
            sections: Vec::new(),
            block_entities: Vec::new(),
            block_ticks: Vec::new(),
            heightmaps: Heightmaps::default(),
            fluid_ticks: Vec::new(),
            inhabited_time: 0,
            structure_data_list: StructureDataList::new(),
        };
        s.create_pregen(heightmap, filler);
        s
    }
    /// creates a new instance of [`ChunkData`]
    ///
    /// [`ChunkData`]: `ChunkData`
    fn create_pregen(&mut self, heightmap: [[u16; 16]; 16], filler: &str) {
        let heightmap = heightmap.map(|map| map.map(|e| e.min(384)));
        let heightmap_max = *heightmap.iter().map(|map| map.iter().min().unwrap_or(&384)).min().unwrap_or(&384) as i16 - 64;
        let lowest_full_section = dbg!(heightmap_max / 16);
        for y in -4..=lowest_full_section {
            let section = ChunkSection::new_filled(y as i8, filler, false);
            self.sections.push(section);
        }
        self.sections.push(ChunkSection::new_with_height_map(lowest_full_section as i8 + 1, filler, true, heightmap.map(|e| e.map(|e|(e as i16) - 64))));
        for y in lowest_full_section+2..=19 {
            let section = ChunkSection::new_filled(y as i8, filler, true);
            self.sections.push(section);
        }
        let motion_blocking_vec: Vec<u16> = heightmap.to_vec().into_iter().flatten().collect();
        let motion_blocking = motion_blocking_vec.chunks(7).map(|chunk| {
            if chunk.len() == 7 {
                ((chunk[0] as u64) << (64-9)) |
                ((chunk[1] as u64) << (64-(9*2))) |
                ((chunk[2] as u64) << (64-(9*3))) |
                ((chunk[3] as u64) << (64-(9*4))) |
                ((chunk[4] as u64) << (64-(9*5))) |
                ((chunk[5] as u64) << (64-(9*6))) |
                ((chunk[6] as u64) << (64-(9*7)))
            } else {
                (chunk[0] as u64) << (64 - 9)
            }
        }).collect::<Vec<_>>().try_into().unwrap();
        self.heightmaps = Heightmaps { 
            motion_blocking,
            motion_blocking_no_leaves: None,
            ocean_floor: None,
            ocean_floor_wg: None,
            world_surface: motion_blocking,
            world_surface_wg: None
        };
        /*
        Self {
            x_pos: x as i32,
            y_pos: -4,
            z_pos: z as i32,
            status: GenerationStatus::Full,
            last_update: current_tick,
            sections,
            block_entities: Vec::new(),
            fluid_ticks: Vec::new(),
            block_ticks: Vec::new(),
            inhabited_time: 0,
            structure_data_list: StructureDataList { structure_references: Vec::new(), starts: Vec::new() },
            heightmaps,
        }
*/
    }
}
/// Struct to hold all structure data of a Chunk
pub struct StructureDataList {
    /// A list of the structure coordinates withing one chunk
    ///
    /// # Note
    ///
    /// In NBT it's represented as an i64
    /// (x << 32) | y
    pub structure_references: Vec<(String, Vec<[u32; 2]>)>,
    ///  Structures that are yet to be generated, stored by general type. Some parts of the
    ///  structures may have already been generated. Completely generated structures are removed by
    ///  setting their id to "INVALID" and removing all other tags.
    pub starts: Vec<(String, ChunkDataHolder)>,
}
impl FromNbtValue for StructureDataList {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        let structure_references: Vec<(String, Vec<[i32;2]>)> = unwrap_to_empty!(data.get("structure_references"), compound).1
            .into_iter()
            .map(|(s, d)|{
                combine_result(s, d.as_i64_array().map_err(|_|()))
            }).collect::<Result<Vec<(String, Vec<i64>)>, ()>>()?
            .into_iter()
            .map(|(s, data)|(s, data.into_iter().map(|data|[(data >> 32) as i32, (data & 0xFFFFFFFF) as i32]).collect()))
            .collect();
        let starts = unwrap_to_empty!(data.get("starts"), compound).1.into_iter()
            .map(|(s, data)|(s, ChunkDataHolder::from(s, data)));
        Err(())
    }
}
#[inline]
fn combine_result(s: String, data: Result<Vec<i64>, ()>) -> Result<(String, Vec<i64>), ()> {
    match data {
        Ok(data) => Ok((s, data)),
        Err(()) => Err(()),
    }
}
impl StructureDataList {
    /// creates a new structure data list
    pub fn new() -> Self {
        Self {
            structure_references: Vec::new(),
            starts: Vec::new(),
        }
    }
}
impl AsNbtValue for StructureDataList {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        use nbt_lib::NbtValue::*;
        use std::string::String as Str;
        let mut references_hash_map = HashMap::new();
        for struct_ref in self.structure_references.iter() {
            references_hash_map.insert(struct_ref.0.to_owned(), LongArray(struct_ref.1.iter().map(|e| ((e[0] as i64) << 32) | e[1] as i64).collect()));
        }
        Ok(Compound(None, create_compound_map!(
            References: Compound(None, references_hash_map),
            starts: Compound(None, self.starts.iter().map(|(s, d)| d.as_nbt_value().map(|v| (s.to_owned(), v))).collect::<Result<HashMap<Str, NbtValue>, ()>>()?)
        )))
    }
}
impl FromNbtValue for ChunkData {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        use nbt_lib::NbtValue::*;
        // use std::string::String as Str;
        if let Compound(_, data) = value {
            let x_pos: Result<&NbtValue, ()> = data.get("xPos").ok_or(());
            return Ok(Self {
                x_pos: unwrap_to_empty!(data.get(Self::X_POS), i32),
                y_pos: unwrap_to_empty!(data.get(Self::Y_POS), i32),
                z_pos: unwrap_to_empty!(data.get(Self::Z_POS), i32),
                status: GenerationStatus::from_str(unwrap_to_empty!(data.get(Self::STATUS), str))?,
                last_update: unwrap_to_empty!(data.get(Self::LAST_UPDATE), i64),
                sections: convert_list_to!(data.get(Self::SECTIONS), ChunkSection),
                block_entities: convert_list_to!(data.get(Self::BLOCK_ENTITIES), BlockEntity),
                heightmaps: convert_list_to!(data.get(Self::HEIGHTMAPS), Heightmaps),
                fluid_ticks: convert_list_to!(data.get(Self::FLUID_TICKS), TileTick),
                block_ticks: convert_list_to!(data.get(Self::BLOCK_TICKS), TileTick),
                inhabited_time: unwrap_to_empty!(data.get(Self::INHABITED_TIME), i64),
                structure_data_list: StructureDataList::from_nbt_value(data.get(Self::STRUCTURES).ok_or(())?),
            })
        }
        Err(())
    }
}
pub(crate) fn list_to_nbt_value_list<T>(data: &Vec<T>) -> Result<NbtValue, ()> where T: AsNbtValue {
    Ok(NbtValue::List(data.iter().map(|i|i.as_nbt_value()).collect::<Result<Vec<NbtValue>, _>>()?))
}
impl AsNbtValue for ChunkData {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        use nbt_lib::NbtValue::*;
        use std::string::String as Str;
        // name is `Some("")` not `None`, because ChunkData is the outer most NbtValue
        Ok(Compound(Some(Str::new()), create_compound_map!( 
            DataVersion: Int(nbt_lib::NBT_VERSION),
            xPos: Int(self.x_pos),
            zPos: Int(self.z_pos),
            yPos: Int(self.y_pos),
            Status: String(self.status.to_string()),
            LastUpdate: Long(self.last_update),
            sections: list_to_nbt_value_list(&self.sections)?,
            block_entities: list_to_nbt_value_list(&self.block_entities)?,
            Heightmaps: self.heightmaps.as_nbt_value()?,
            fluid_ticks: list_to_nbt_value_list(&self.fluid_ticks)?,
            block_ticks: list_to_nbt_value_list(&self.block_ticks)?,
            InhabitedTime: Long(self.inhabited_time),
            // PostProcessing: list_to_nbt_value_list(&self.po)
            structures: self.structure_data_list.as_nbt_value()?
        )))
    }
}
/// This enum is to determine, if a structure is in a chunk or not
pub enum ChunkDataHolder {
    /// This option is used if the structure is in the chunk
    Data(structure::StructureData),
    /// This optiojn is used, if the structure is marked as abscent
    Empty {
        /// The id is INVALID, if it is absent, its only for parsing reasons included
        id: String,
    }
}
impl ChunkDataHolder {
    pub fn from(name: String, value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        if unwrap_to_empty!(data.get("id"), str) == "INVALID" {
            return Ok(Self::Empty { id: String::from("INVALID") });
        }
        Ok(Self::Data(structure::StructureData::from(name, data)?))
    }
}
impl AsNbtValue for ChunkDataHolder {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        todo!()
    }
}
pub mod structure;
#[deprecated]
pub(crate) struct TileTick;
impl AsNbtValue for TileTick {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        todo!()
    }
}
#[deprecated]
pub(crate) struct ToBeTicked;
impl AsNbtValue for ToBeTicked {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        todo!()
    }
}
impl IntoNbt for ChunkData {
    fn to_nbt(&self) -> nbt_lib::NbtValue {
        let data_version = NbtValue::Int(nbt_lib::NBT_VERSION);
        todo!()
    }
}
