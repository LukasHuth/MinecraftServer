use std::ops::{Deref, DerefMut};

use super::WaterAnimal;

/// An instance of a dolphin
pub struct Dolphin {
    water_animal: WaterAnimal,
    /// The position of its treasure
    pub treasure_position: (u32, u32, u16),
    /// Whether it has a fish or not
    pub has_fish: bool,
    /// The moisture level of the dolphin
    pub moisture_level: i32,
}
impl Deref for Dolphin {
    type Target = WaterAnimal;

    fn deref(&self) -> &Self::Target {
        &self.water_animal
    }
}
impl DerefMut for Dolphin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal
    }
}
impl Default for Dolphin {
    fn default() -> Self {
        Self {
            water_animal: WaterAnimal::default(),
            treasure_position: (0,0,0),
            has_fish: false,
            moisture_level: 2400,
        }
    }
}
