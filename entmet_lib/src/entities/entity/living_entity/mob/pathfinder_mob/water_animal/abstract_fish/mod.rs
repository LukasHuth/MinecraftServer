use std::ops::{Deref, DerefMut};

use super::WaterAnimal;
pub mod cod;
pub mod puffer_fish;
pub mod salmon;
pub mod tropical_fish;
pub mod tadpole;

pub struct AbstractFish {
    water_animal: WaterAnimal,
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
