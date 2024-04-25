//! This is a module containing all important data structs for storing the data of a chunk section

use nbt_lib::NbtValue;

/// A struct with block data
pub struct BlockData {
    /// The name of the block
    pub name: String,
    /// The properties of the block
    pub properties: NbtValue,
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
    pub block_data: Option<[u64; 4096]>,
    /// A list of all used biomes in the chunk
    pub biome_palette: Vec<String>,
    /// An optional list of the biomes used on each (x | z) position. If the value is `None` every
    /// location in the section has the same biome
    pub biome_data: Option<[u64; 64]>,
    /// The light emitter data of each block in the chunk
    ///
    /// # Note
    ///
    /// In the Nbt data this will be stored as a [u8; 2048] where each element will contain two
    /// block each 4-bits of light data.
    pub block_light: [u8; 4096],
    /// The sky light data of each block in the chunk
    ///
    /// # Note
    ///
    /// In the Nbt data this will be stored as a [u8; 2048] where each element will contain two
    /// block each 4-bits of light data.
    pub sky_light: [u8;4096],
}
