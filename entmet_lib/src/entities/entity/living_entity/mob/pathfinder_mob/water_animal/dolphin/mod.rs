use std::ops::{Deref, DerefMut};

use super::WaterAnimal;

pub struct Dolphin {
    water_animal: WaterAnimal,
    pub treasure_position: (u32, u32, u16),
    pub has_fish: bool,
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
