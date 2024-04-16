use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum AxolotlVariant {
    #[default] Lucy = 0,
    Wild = 1,
    Gold = 2,
    Cyan = 3,
    Blue = 4,
}
#[derive(Default)]
pub struct Axolotl {
    animal: Animal,
    pub variant: AxolotlVariant,
    pub is_playing_dead: bool,
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
