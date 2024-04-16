use std::ops::{Deref, DerefMut};

use super::TameableAnimal;

#[repr(u8)]
#[derive(Default)]
pub enum ParrotVariant {
    #[default] RedBlue = 0,
    Blue = 1,
    Green = 2,
    YellowBlue = 3,
    Grey = 4,
}

#[derive(Default)]
pub struct Parrot {
    tameable_animal: TameableAnimal,
    pub variant: ParrotVariant,
}
impl Deref for Parrot {
    type Target = TameableAnimal;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal
    }
}
impl DerefMut for Parrot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal
    }
}
