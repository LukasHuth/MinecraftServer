//! This Module provides datastructures for chunk data
//!
//! # Sources
//! - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format)

use std::io::{Cursor, Read as _, Write};

use binary_utils::SyncDataWriter;
use flate2::{read::{GzEncoder, ZlibEncoder}, Compression};
use nbt_lib::{traits::IntoNbt, version::Java, NbtValue};

use super::CompressionScheme;

/// A struct containing the chunk data
pub struct Chunk {
    /// length of the data in bytes
    pub length: u32,
    /// The used compression
    pub compression: CompressionScheme,
    /// The compressed data as nbt data
    pub data: ChunkData,
}
impl SyncDataWriter for Chunk {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        if let Err(_) = writer.write_all(&self.length.to_be_bytes()) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        if let Err(_) = writer.write_all(&[self.compression as u8]) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        let data = self.data.to_nbt().to_binary::<Java>();
        let data = if let Ok(data) = data { data } else { return Err(binary_utils::Error::FailedToWrite); };
        let mut compressed_data: Vec<u8> = Vec::new();
        match self.compression {
            CompressionScheme::None => compressed_data = data,
            CompressionScheme::LZ4 => todo!(),
            CompressionScheme::Gzip => {
                let cursor = Cursor::new(data);
                let mut encoder = GzEncoder::new(cursor, Compression::default());
                if let Err(_) = encoder.read(&mut compressed_data) {
                    return Err(binary_utils::Error::FailedToWrite);
                }
            }
            CompressionScheme::Zlib => {
                let cursor = Cursor::new(data);
                let mut encoder = ZlibEncoder::new(cursor, Compression::default());
                if let Err(_) = encoder.read(&mut compressed_data) {
                    return Err(binary_utils::Error::FailedToWrite);
                }
            }
        }
        if let Err(_) = writer.write_all(&compressed_data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
/// Enum of all Generation states
pub enum GenerationStatus {
    /// Chunk is not generated
    Empty,
    /// The structures where started to being generated
    StructureStarts,
    /// IDK, hopefully i update this the moment i finished the world generation an know what step
    /// this is
    StructureReferences,
    /// The biomes are started to being generated
    Biomes,
    /// The noise for the chunk is generated
    Noise,
    /// The surface of the chunk is generated
    Surface,
    /// The Caves of the chunk are generated? (guessing, because i didn't worked at the world
    /// generation at the point of writing this code)
    Carvers,
    /// Liquid generation ig? 
    LiquidCarvers,
    /// Generating features?
    Features,
    /// Generating light information
    Light,
    /// Generation spawn information
    Spawn,
    /// Generating the heightmaps of the chunk
    Heightmaps,
    /// The cunk is fully generated
    Full,
}
/// A struct with block data
pub struct BlockData {
    /// The name of the block
    pub name: String,
    /// The properties of the block
    pub properties: NbtValue,
}
/// A struct storing block entity data
///
/// These are blocks, that store more data than normal blocks. e.g. Chest. These blocks where
/// called "tile entities" until they where renamed in 1.18
///
/// # Sources
/// - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format#Block_entity_format)
pub struct BlockEntity {
}
/// A structure containing important data of a section of a `Chunk`
/// it stores a section of 16*16*16 (4096) blocks
///
/// # Source
/// - [fandom.minecraft.com](https://minecraft.fandom.com/wiki/Chunk_format#NBT_structure)
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
    // Info: CarvingMasks are only used for proto chunk and will probably be added to this the
    // moment i write the world generation and learn, how and what exactly it stores
    /// All heightmaps of the section
    pub heightmaps: Heightmaps,
}
/// Definition of the layout of an heightmap
///
/// # Note
///
/// Every element(u64) stores 7 values each of them 9-bits long
pub type Heightmap = [u64;37];
/// A Struct holding all Heightmaps
pub struct Heightmaps {
    /// A Heightmap of the heigest motion blocking block in the section
    pub motion_blocking: Heightmap,
    /// A Heightmap of the heigest motion blocking block in the section, excluding leaves
    pub motion_blocking_no_leaves: Heightmap,
    /// A Heightmap of the heigest ocean floor block in the section
    pub ocean_floor: Heightmap,
    /// A Heightmap of the heigest ocean floor block in the section (use case of this is currently
    /// unknown to me)
    pub ocean_fllor_wg: Heightmap,
    /// A Heightmap of the heigest world surface block in the section
    pub world_surface: Heightmap,
    /// A Heightmap of the heigest world surface block in the section (use case of this is currently
    /// unknown to me)
    pub world_surface_wg: Heightmap,
}
/// A struct holding all chunk data
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
    pub sections: Vec<ChunkSection>,
}
impl IntoNbt for ChunkData {
    fn to_nbt(&self) -> nbt_lib::NbtValue {
        let data_version = NbtValue::Int(nbt_lib::NBT_VERSION);
        todo!()
    }
}
