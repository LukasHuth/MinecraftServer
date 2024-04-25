use std::ops::{Deref, DerefMut};

use crate::datatypes::MinecraftColor;

use super::TameableAnimal;

/// An instance of a wolf
#[derive(Default)]
pub struct Wolf {
    tameable_animal: TameableAnimal,
    /// Whether it is begging or not
    pub begging: bool,
    /// The color of its collar
    pub collar_color: MinecraftColor,
    /// The amount of ticks, that it still is angry
    pub anger_time: i32,
}
impl Deref for Wolf {
    type Target = TameableAnimal;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal
    }
}
impl DerefMut for Wolf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal
    }
}
