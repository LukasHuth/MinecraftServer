//! This is a module containing all important data structs for storing the data of a chunk section

use std::{collections::HashMap, slice};

use nbt_lib::{traits::{AsNbtValue, FromNbtValue}, NbtValue};

use crate::{anvil::region::chunk::chunk_data::list_to_nbt_value_list, convert_list_to, create_compound_map, unwrap_to_empty};

/// A struct with block data
pub struct BlockData {
    /// The name of the block
    pub name: String,
    /// The properties of the block
    pub properties: NbtValue,
}
impl FromNbtValue for BlockData {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        let name = unwrap_to_empty!(data.get("Name"), string);
        let properties = data.get("Properties").ok_or(())?.clone();
        Ok(Self { name, properties })
    }
}
impl AsNbtValue for BlockData {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        use nbt_lib::NbtValue::*;
        Ok(Compound(None, create_compound_map!(
            Name: String(self.name.clone()),
            Properties: self.properties.clone()
        )))
    }
}

/// A structure containing important data of a section of a `Chunk`
/// it stores a section of 16*16*16 (4096) blocks
///
/// # Source
/// - [fandom.minecraft.com](https://minecraft.fandom.com/wiki/Chunk_format#NBT_structure)
///
/// # Info
///
/// Everything, taged as "Not confirmed for 1.18 format" is not implemented, but will be if it
/// turnes out to be neccessarry
pub struct ChunkSection {
    /// The y position of the sector
    pub y: i8,
    /// A list of all used blocks in a chunk
    pub block_palette: Vec<BlockData>,
    /// A list of all placed blocks, the list contains the offset into the `block_palette` list to
    /// save some memmory, this field is optional, if `None` and `block_palette.len() == 1` then
    /// the whole sections is filled with that one block.
    ///
    /// # Example
    ///
    /// full chunk of Air:
    /// the `block_palette` contains the data for air. the `block_data` is `None` to indicate that
    /// the same data is used accross the whole section
    ///
    /// # Info
    ///
    /// The stored  indices are always as small as possible, but at least 4 bits with no packing
    /// accross multiple elements of the array. Meaning if the chunk consists of 33 different
    /// blocks, we need 5 bits to represent all unique blocks, now we can divide 64 by the amount
    /// of needed bits and floor the result to know how much block entries we can store per
    /// element(u64). For example $⌊64/5⌋ = 12$ so if we need 5 bits to represent each unique block
    /// we can store 12 per element.
    ///
    /// # Note
    ///
    /// The explanation above only applies to the nbt representation. The used on in the code has
    /// an entry for each element to keep the amount of computation low in a trade of with a bit
    /// more used memory
    pub block_data: Option<[i64; 4096]>,
    /// A list of all used biomes in the chunk
    pub biome_palette: Vec<String>,
    /// An optional list of the biomes used on each (x | z) position. If the value is `None` every
    /// location in the section has the same biome
    pub biome_data: Option<[i64; 64]>,
    /// The light emitter data of each block in the chunk
    ///
    /// # Note
    ///
    /// In the Nbt data this will be stored as a [u8; 2048] where each element will contain two
    /// block each 4-bits of light data.
    pub block_light: [i8; 4096],
    /// The sky light data of each block in the chunk
    ///
    /// # Note
    ///
    /// In the Nbt data this will be stored as a [u8; 2048] where each element will contain two
    /// block each 4-bits of light data.
    pub sky_light: [i8;4096],
}
impl ChunkSection {
    const Y: &'static str = "Y";
    const BLOCK_STATES: &'static str = "block_states";
    const BIOMES: &'static str = "biomes";
    const BLOCK_LIGHT: &'static str = "BlockLight";
    const SKY_LIGHT: &'static str = "SkyLight";
    pub fn new_filled(y: i8, filler: &str, sees_sky: bool) -> Self {
        let block_palette = vec![BlockData{name: filler.to_owned(), properties: NbtValue::Compound(None, HashMap::new())}];
        let sky_light = if !sees_sky {
            [0; 4096]
        } else {
            let mut data = [0; 4096];
            data.iter_mut()
                .enumerate()
                .filter(|(i, _)| *i >= 16 * 16 * 15 && *i < 16 * 16 * 15 + 16)
                .for_each(|(_, v)| *v = 15);
            data
        };
        Self {
            y,
            block_palette,
            block_data: None,
            biome_palette: vec!["minecraft:plains".to_string()],
            biome_data: None,
            block_light: [0; 4096],
            sky_light
        }
    }
    pub fn new_with_height_map(y: i8, filler: &str, sees_sky: bool, heightmap: [[i16; 16]; 16]) -> Self {
        let block_palette = vec![
            BlockData{name: "minecraft:air".to_owned(), properties: NbtValue::Compound(None, HashMap::new())},
            BlockData{name: filler.to_owned(), properties: NbtValue::Compound(None, HashMap::new())},
        ];
        let heightmap_offset = y * 16;
        let sky_light = if !sees_sky {
            [0; 4096]
        } else {
            let mut data = [0; 4096];
            data.iter_mut()
                .enumerate()
                .filter(|(i, _)|{
                    let (x, z, y) = (*i & 15, (*i >> 4) & 15, *i as i64 / (16*16) + heightmap_offset as i64);
                    heightmap[z][x] as i64 <= y
                })
                .for_each(|(_, v)| *v = 15);
            data
        };
        let block_data = {
            let mut data = [1; 4096];
            data.iter_mut()
                .enumerate()
                .filter(|(i, _)|{
                    let (x, z, y) = (*i & 15, (*i >> 4) & 15, *i as i64 / (16*16) + heightmap_offset as i64);
                    (heightmap[z][x] as i64) < y
                })
                .for_each(|(_, v)| *v = 0);
            Some(data)
        };
        Self {
            y,
            block_palette,
            block_data,
            biome_palette: vec!["minecraft:plains".to_string()],
            biome_data: None,
            block_light: [0; 4096],
            sky_light
        }
    }
}
impl FromNbtValue for ChunkSection {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        use nbt_lib::NbtValue::*;
        if let Compound(_, data) = value {
            let (biome_data, biome_palette) = get_biome(unwrap_to_empty!(data.get(Self::BIOMES), compound).1)?;
            let (block_data, block_palette) = get_block_states(unwrap_to_empty!(data.get(Self::BLOCK_STATES), compound).1)?;
            let block_light = unwrap_to_empty!(data.get(Self::BLOCK_LIGHT), i8_array).try_into().unwrap();
            let sky_light = unwrap_to_empty!(data.get(Self::SKY_LIGHT), i8_array).try_into().unwrap();
            let y = unwrap_to_empty!(data.get(Self::Y), i8);
            return Ok(Self {
                biome_data,
                biome_palette,
                block_palette,
                block_data,
                y,
                block_light,
                sky_light,
            })
        }
        Err(())
    }
}
struct BiomePalette(String);
impl FromNbtValue for BiomePalette {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        Ok(Self(unwrap_to_empty!(Some(value), string)))
    }
}
fn get_biome(values: HashMap<String, NbtValue>) -> Result<(Option<[i64; 64]>, Vec<String>), ()> {
    let biomes: Vec<NbtValue> = unwrap_to_empty!(values.get("palette"), list);
    let biomes: Result<Vec<String>, ()> = biomes.into_iter().map(|value| value.as_string().map_err(|_|())).collect();
    let biomes = biomes?;
    let biomes_data = if values.contains_key("data") { Some(unwrap_to_empty!(values.get("data"), i64_array).try_into().unwrap()) } else { None };
    Ok((biomes_data, biomes))
}
fn get_block_states(values: HashMap<String, NbtValue>) -> Result<(Option<[i64; 4096]>, Vec<BlockData>), ()> {
    let palette: Option<&NbtValue> = values.get("palette");
    let palette: Vec<NbtValue> = unwrap_to_empty!(palette, list);
    let palette = palette.into_iter().map(|value| BlockData::from_nbt_value(value)).collect::<Result<_, _>>()?;
    let block_data = if values.contains_key("data") { Some(unwrap_to_empty!(values.get("data"), i64_array).try_into().unwrap()) } else { None };
    Ok((block_data, palette))
}
impl AsNbtValue for ChunkSection {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        use nbt_lib::NbtValue::*;
        Ok(Compound(None, create_compound_map!(
            Y: Byte(self.y),
            block_states: Compound(None, create_compound_map!(
                palette: list_to_nbt_value_list(&self.block_palette)?,
                data: LongArray(self.block_data.map(|e|e.to_vec()).unwrap_or(Vec::new()))
            )),
            biomes: Compound(None, create_compound_map!(
                palette: List(self.biome_palette.iter().map(|s| nbt_lib::NbtValue::String(s.to_owned())).collect()),
                data: LongArray(self.biome_data.map(|e|e.to_vec()).unwrap_or(Vec::new()))
            )),
            BlockLight: ByteArray(self.block_light.chunks_exact(2).map(|e|(e[0].min(0xF) << 4) | e[1].min(0xF)).collect()),
            SkyLight: ByteArray(self.sky_light.chunks_exact(2).map(|e|(e[0].min(0xF) << 4) | e[1].min(0xF)).collect())
        )))
    }
}
