use std::ops::{Deref, DerefMut};

use super::AbstractFish;

#[derive(Default)]
pub struct Salmon {
    water_animal: AbstractFish,
}
impl Deref for Salmon {
    type Target = AbstractFish;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for Salmon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
