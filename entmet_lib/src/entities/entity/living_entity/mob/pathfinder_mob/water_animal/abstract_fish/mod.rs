use std::ops::{Deref, DerefMut};

use super::WaterAnimal;
mod cod;
pub use cod::*;
mod puffer_fish;
pub use puffer_fish::*;
mod salmon;
pub use salmon::*;
mod tropical_fish;
pub use tropical_fish::*;
mod tadpole;
pub use tadpole::*;

/// An interface of a fish
#[derive(PartialEq)]
pub struct AbstractFish {
    water_animal: WaterAnimal,
    /// whether the fish is from a bucket or not
    pub from_bucket: bool,
}
impl Deref for AbstractFish {
    type Target = WaterAnimal;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for AbstractFish {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
impl Default for AbstractFish {
    fn default() -> Self {
        Self {
            water_animal: WaterAnimal::default(),
            from_bucket: false,
        }
    }
}
