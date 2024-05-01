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
    /// The chunk z coordinate of this structure
    pub chunk_z: i32,
    /// The id of the structure
    pub id: String,
    /// A list where the structure already generated. Only used in monument
    pub processed: Option<Vec<(i32, i32)>>,
    /// (Village only) Whether the village generated at least 3 non-roads.
    pub valid: Option<bool>,
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
