use std::ops::{Deref, DerefMut};

use super::AbstractFish;

/// An instance of a Puffer fish
#[derive(Default)]
pub struct PufferFish {
    water_animal: AbstractFish,
    /// The state of the puffer fish, ranging from 0 to 2
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
