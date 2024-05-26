use std::str::FromStr;

use serde::{Deserialize, Serialize};


/// Enum of all Generation states
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
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
                if s.len() > 10 && &s[..10] == "minecraft:" {
                    Self::from_str(&s[10..])
                } else {
                    Err(())
                }
            }
        }
    }
}
