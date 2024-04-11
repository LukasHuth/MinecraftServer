use std::ops::{Deref, DerefMut};

use super::AbstractFish;

#[derive(Default)]
pub struct Tadpole {
    water_animal: AbstractFish,
}
impl Deref for Tadpole {
    type Target = AbstractFish;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for Tadpole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
