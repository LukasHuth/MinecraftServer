use std::ops::{Deref, DerefMut};

use super::AbstractFish;

/// An instance of a cod
#[derive(PartialEq, Default)]
pub struct Cod {
    water_animal: AbstractFish,
}
impl Deref for Cod {
    type Target = AbstractFish;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for Cod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
