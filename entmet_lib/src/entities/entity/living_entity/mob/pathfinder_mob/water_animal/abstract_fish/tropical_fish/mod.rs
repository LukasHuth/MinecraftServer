use std::ops::{Deref, DerefMut};

use crate::datatypes::MinecraftColor;

use super::AbstractFish;

/// An pattern of a tropical fish
///
/// # Source
/// - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Tropical_Fish#ID)
#[repr(u64)]
#[allow(missing_docs)]
#[derive(Default, Clone, Copy)]
pub enum FishPatternAndSize {
    #[default] Kob = 0x0000,
    Flopper = 0x0001,
    Sunstreak = 0x0100,
    Stripey = 0x0101,
    Snooper = 0x0200,
    Glitter = 0x0201,
    Dasher = 0x0300,
    Blockfish = 0x0301,
    Brinely = 0x0400,
    Betty = 0x0401,
    Spotty = 0x0500,
    Clayfish = 0x0501,
}
/// A struct holding fish variant information
pub struct FishVariant {
    /// The pattern of the fish
    pub pattern: FishPatternAndSize,
    /// The color of the body
    pub body_color: MinecraftColor,
    /// The color of the pattern
    pub pattern_color: MinecraftColor,
}
impl Default for FishVariant {
    fn default() -> Self {
        Self {
            pattern: FishPatternAndSize::default(),
            body_color: MinecraftColor::White,
            pattern_color: MinecraftColor::White,
        }
    }
}
/// An instance of a tropical fish
#[derive(Default)]
pub struct TropicalFish {
    water_animal: AbstractFish,
    /// The variant of the fish
    pub variant: FishVariant,
}
impl Deref for TropicalFish {
    type Target = AbstractFish;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for TropicalFish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
