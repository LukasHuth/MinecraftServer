//! Module for structure data inside of a chunk

/// A struct for structure data
pub struct StructureData {
    /// The six coordinated of the bounding box lower x, y, z and upper x, y, z
    pub bounding_box: [i32;6],
    /// The biome id the structure is in
    pub biome: String,
    /// A list of all structure pieces
    pub children: Vec<StructurePiece>,
    /// The chunk x coordinate of this structure
    pub chunk_x: i32,
    /// The chunk x coordinate of this structure
    pub chunk_y: i32,
    /// The id of the structure
    pub id: String,
    /// A list where the structure already generated. Only used in monument
    pub processed: Option<Vec<(i32, i32)>>,
    /// (Village only) Whether the village generated at least 3 non-roads.
    pub valid: bool,
}
/// A struct for a piece of a structure
#[deprecated]
pub struct StructurePiece {
    /// The six coordinated of the bounding box lower x, y, z and upper x, y, z
    pub bounding_box: [i32;6],
}
