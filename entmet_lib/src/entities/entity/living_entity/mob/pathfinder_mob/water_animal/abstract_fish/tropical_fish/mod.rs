use std::ops::{Deref, DerefMut};

use super::AbstractFish;

#[derive(Default)]
pub struct TropicalFish {
    water_animal: AbstractFish,
    pub variant: i32,
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
