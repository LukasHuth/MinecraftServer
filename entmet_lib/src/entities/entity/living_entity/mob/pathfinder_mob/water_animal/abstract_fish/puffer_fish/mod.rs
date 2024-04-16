use std::ops::{Deref, DerefMut};

use super::AbstractFish;

#[derive(Default)]
pub struct PufferFish {
    water_animal: AbstractFish,
    pub puff_state: i32,
}
impl Deref for PufferFish {
    type Target = AbstractFish;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for PufferFish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
