use std::ops::{Deref, DerefMut};

use super::Animal;

/// An enum of all axolotl variants
#[repr(u8)]
#[allow(missing_docs)]
#[derive(PartialEq, Default)]
pub enum AxolotlVariant {
    #[default] Lucy = 0,
    Wild = 1,
    Gold = 2,
    Cyan = 3,
    Blue = 4,
}
/// An instance of an axolotl
#[derive(PartialEq, Default)]
pub struct Axolotl {
    animal: Animal,
    /// The variant of the axolotl
    pub variant: AxolotlVariant,
    /// Whether it is playing dead or not
    pub is_playing_dead: bool,
    /// Whether it is spawned from a fucket or not
    pub spawned_from_bucket: bool,
}
impl Deref for Axolotl {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Axolotl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
