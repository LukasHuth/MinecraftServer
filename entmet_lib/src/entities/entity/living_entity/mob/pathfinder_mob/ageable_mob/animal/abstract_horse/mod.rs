use std::ops::{Deref, DerefMut};

use super::Animal;

pub mod horse;
pub mod zombie_horse;
pub mod skeleton_horse;
pub mod camel;
pub mod chested_horse;

#[repr(u8)]
pub enum HorseInfo {
    IsTame = 0x02,
    IsSaddled = 0x04,
    HasBred = 0x08,
    IsEating = 0x10,
    IsRearing = 0x20,
    IsMouthOpen = 0x40,
}

#[derive(Default)]
pub struct AbstractHorse {
    animal: Animal,
    pub info: Vec<HorseInfo>,
}
impl Deref for AbstractHorse {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for AbstractHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
