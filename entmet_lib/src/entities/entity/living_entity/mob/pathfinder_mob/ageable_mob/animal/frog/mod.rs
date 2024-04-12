use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum FrogVariant {
    #[default] Temperate = 0,
    Warm = 1,
    Cold = 2,
}

#[derive(Default)]
pub struct Frog {
    animal: Animal,
    pub variant: FrogVariant,
    pub togue_target: Option<i32>,
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
