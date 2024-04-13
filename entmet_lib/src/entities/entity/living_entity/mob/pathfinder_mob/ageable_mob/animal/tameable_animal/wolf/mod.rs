use std::ops::{Deref, DerefMut};

use crate::datatypes::MinecraftColor;

use super::TameableAnimal;

#[derive(Default)]
pub struct Wolf {
    tameable_animal: TameableAnimal,
    pub begging: bool,
    pub collar_color: MinecraftColor,
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
