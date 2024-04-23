/// A struct, dedicated to hold informations about an dimension
pub struct Dimension {
    /// The id of the dimension type
    pub type_name: String,
    /// Information how to generate the dimension
    pub generator: Generator,
}
/// An enum of types that a generator can have
#[allow(missing_docs)]
pub enum GeneratorType {
    Noise,
    Flat,
    Debug
}
impl GeneratorType {
    /// function to get the object as a string
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Noise => "noise",
            Self::Flat => "flat",
            Self::Debug => "debug",
        }
    }
}
/// An enum of types that a generator biome source can have
#[allow(missing_docs)]
pub enum BiomeSourceType {
    MultiNoise,
    Fixed,
    Checkerboard,
    TheEnd,
}
impl BiomeSourceType {
    /// function to get the object as a string
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::MultiNoise => "multi_noise",
            Self::TheEnd => "the_end",
            Self::Fixed => "fixed",
            Self::Checkerboard => "checkerboard",
        }
    }
}
/// A struct storing data important for noise generation
///
/// # Note
///
/// This data is stored as json data
///
/// # Info
///
/// This one is marked as deprecated, to remember that it is not fully implemented
#[deprecated]
pub struct NoiseSettings {
}
/// Struct holding data, how to generate a dimension
pub enum Generator {
    /// Used for debug dimensions
    Debug,
    /// Used for dimensions generated with noise
    Noise(NoiseGenerator),
    /// Used for flat dimensions
    Flat(FlatGenerator),
}
/// An enum storing data to generate a dimension using noise
pub enum NoiseGenerator {
    /// Dimension generated with multiple noise data
    MultiNoise(MultiNoiseGenerator),
    /// Dimension generated like the end
    TheEnd,
    /// Dimension consisting of one single biome
    SingleBiome(String),
    /// Dimension looking like a checkerboard of biomes
    Checkerboard(CheckerboardGenerator),
}
/// A struct used to store data for a multi noise dimension
pub struct MultiNoiseGenerator {
    /// preset data
    pub preset: String,
    /// A list off all used biomes
    pub biomes: Vec<biome::Biome>,
}
pub mod biome;
/// A struct used to store data for a checkerboard dimension
pub struct CheckerboardGenerator {
    /// Biome data
    pub biomes: ListIdOrTag<biome::Biome>,
    /// The scale of the biome
    ///
    /// # Note
    ///
    /// Range is 0..=62 with a default to 2
    pub scale: Option<i32>,
}
/// Enumumerate for elements that can be a list, an id or a tag
pub enum ListIdOrTag<T> {
    /// A list of ids
    IdList(Vec<String>),
    /// A single id
    SingleId(String),
    /// A single represented as a tag
    SingleTag(T),
}
/// A struct storing data to generate a flag world
pub struct FlatGenerator {
    /// The layers ob the flat generator
    pub layers: Vec<SuperflatLayer>,
    /// Optional id of the biome
    pub biome: Option<String>,
    /// Whether to generate lava lakes or not
    ///
    /// # Default
    ///
    /// false
    pub lakes: bool,
    /// Whether to generate biome specific features
    pub features: bool,
    /// Opptional structure generation overrides
    pub structure_overrides: Option<ListIdOrTag<StructureOverride>>,
}
/// Placeholder struct for structure override tags
#[deprecated]
pub struct StructureOverride;
/// Layer data
pub struct SuperflatLayer {
    /// The height of the layer
    pub height: i32,
    /// The block of the layer
    ///
    /// # Note
    ///
    /// Defaults to `minecraft:air`
    pub block: Option<String>,
}
