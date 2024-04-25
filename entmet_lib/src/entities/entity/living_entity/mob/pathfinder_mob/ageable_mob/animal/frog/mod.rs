use std::ops::{Deref, DerefMut};

use super::Animal;

/// An enum of all frog variants
#[repr(u8)]
#[derive(Default)]
pub enum FrogVariant {
    /// Normal variant
    #[default] Temperate = 0,
    /// Warm variant
    Warm = 1,
    /// Cold variant
    Cold = 2,
}

/// An instance of a Frog
#[derive(Default)]
pub struct Frog {
    animal: Animal,
    /// The variant of the frog
    pub variant: FrogVariant,
    /// The target id of the entity that the frog is focosing
    pub tongue_target: Option<i32>,
}
impl Deref for Frog {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Frog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
