use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

/// An enum of all color variants of a horse
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(u64)]
pub enum HorseColor {
    #[default] White = 0,
    Creamy = 1,
    Chestnut = 2,
    Brown = 3,
    Black = 4,
    Gray = 5,
    DarkBrown = 6,
}
/// An enum of possible markings of a horse
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(u64)]
pub enum HorseMarking {
    #[default] None = 0,
    WhiteStockingsAndBlaze = 256,
    WhitePatches = 512,
    WhiteDots = 768,
}

/// A struct for holding the data neccessary to construct the horse variant
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HorseVariant {
    /// The color of the horse
    pub color: HorseColor,
    /// The marking that the horse has
    pub marking: HorseMarking,
}
impl Into<i32> for HorseVariant {
    fn into(self) -> i32 {
        ( self.color as u64 | self.marking as u64 ) as i32
    }
}

/// An instance of a horse
#[derive(Default)]
pub struct Horse {
    abstract_horse: AbstractHorse,
    /// The variant of the horse
    pub variant: HorseVariant,
}
impl Deref for Horse {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for Horse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
