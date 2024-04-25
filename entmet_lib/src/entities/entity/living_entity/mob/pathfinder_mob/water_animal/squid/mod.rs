use std::ops::{Deref, DerefMut};

use super::WaterAnimal;

/// An instance of a squid
#[derive(Default)]
pub struct Squid {
    water_animal: WaterAnimal,
}
impl Deref for Squid {
    type Target = WaterAnimal;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for Squid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
