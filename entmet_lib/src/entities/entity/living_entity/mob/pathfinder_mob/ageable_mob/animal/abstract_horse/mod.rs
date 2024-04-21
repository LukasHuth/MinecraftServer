use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Animal;

mod horse;
pub use horse::*;
mod zombie_horse;
pub use zombie_horse::*;
mod skeleton_horse;
pub use skeleton_horse::*;
mod camel;
pub use camel::*;
mod chested_horse;
pub use chested_horse::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HorseInfo {
    IsTame = 0x02,
    IsSaddled = 0x04,
    HasBred = 0x08,
    IsEating = 0x10,
    IsRearing = 0x20,
    IsMouthOpen = 0x40,
}
impl Into<u8> for HorseInfo {
    fn into(self) -> u8 {
        todo!()
    }
}

#[derive(Default)]
pub struct AbstractHorse {
    animal: Animal,
    pub info: Mask<HorseInfo>,
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
