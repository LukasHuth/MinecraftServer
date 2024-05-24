
use nbt_lib::{convert_list_to, traits::{AsNbtValue, FromNbtValue, IntoNbt}, NbtValue};

use std::{collections::HashMap, str::FromStr};

use crate::anvil::region::chunk::section::ChunkSection;
use nbt_lib::{create_compound_map, unwrap_to, unwrap_to_empty};

use super::block_entity::BlockEntity;

mod generation_status;

pub use generation_status::*;

mod heightmaps;

pub use heightmaps::*;

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

mod structure_data_list;

pub use structure_data_list::*;

impl FromNbtValue for ChunkData {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        use nbt_lib::NbtValue::*;
        // use std::string::String as Str;
        if let Compound(_, data) = value {
            let (name, structure_data) = data.get(Self::STRUCTURES).ok_or(())?.as_compound().map_err(|_|())?;
            let structure_data_list_nbt = NbtValue::Compound(name.map(|s|s.clone()), structure_data);
            let structure_data_list = StructureDataList::from_nbt_value(structure_data_list_nbt)?;
            return Ok(Self {
                x_pos: unwrap_to_empty!(data.get(Self::X_POS), i32),
                y_pos: unwrap_to_empty!(data.get(Self::Y_POS), i32),
                z_pos: unwrap_to_empty!(data.get(Self::Z_POS), i32),
                status: GenerationStatus::from_str(unwrap_to_empty!(data.get(Self::STATUS), str))?,
                last_update: unwrap_to_empty!(data.get(Self::LAST_UPDATE), i64),
                sections: convert_list_to!(data.get(Self::SECTIONS), ChunkSection),
                block_entities: convert_list_to!(data.get(Self::BLOCK_ENTITIES), BlockEntity),
                heightmaps: unwrap_to!(data.get(Self::HEIGHTMAPS), Heightmaps),
                fluid_ticks: convert_list_to!(data.get(Self::FLUID_TICKS), TileTick),
                block_ticks: convert_list_to!(data.get(Self::BLOCK_TICKS), TileTick),
                inhabited_time: unwrap_to_empty!(data.get(Self::INHABITED_TIME), i64),
                structure_data_list,
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
        Ok(Self::Data(structure::StructureData::from_nbt(name, data)?))
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
impl FromNbtValue for TileTick {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        todo!()
    }
}
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
