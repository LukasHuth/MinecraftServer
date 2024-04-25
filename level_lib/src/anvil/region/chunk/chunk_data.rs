//! A module for chunk data
use nbt_lib::{traits::IntoNbt, NbtValue};

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
    // Info: CarvingMasks are only used for proto chunk and will probably be added to this the
    // moment i write the world generation and learn, how and what exactly it stores
    /// All heightmaps of the section
    pub heightmaps: Heightmaps,
}
impl IntoNbt for ChunkData {
    fn to_nbt(&self) -> nbt_lib::NbtValue {
        let data_version = NbtValue::Int(nbt_lib::NBT_VERSION);
        todo!()
    }
}
