use std::ops::{Deref, DerefMut};

use crate::datatypes::MinecraftColor;

use super::TameableAnimal;

/// An enum of all cat variants
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Default)]
pub enum CatVariant {
    White = 0,
    Tuxedo = 1,
    Ginger = 2,
    Siamese = 3,
    BritishShorthair = 4,
    Calico = 5,
    Persian = 6,
    Ragdoll = 7,
    Tabby = 8,
    #[default] Black = 9,
    Jellie = 10,
}

/// An instance of a cat
#[derive(Default)]
pub struct Cat {
    tameable_animal: TameableAnimal,
    /// The variant of the cat
    pub variant: CatVariant,
    /// Whether it is lying or not
    pub lying: bool,
    /// Whether it is reaxed or not
    pub relaxed: bool,
    /// The collor of its collar
    pub collar_color: MinecraftColor,
}
impl Deref for Cat {
    type Target = TameableAnimal;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal
    }
}
impl DerefMut for Cat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal
    }
}
