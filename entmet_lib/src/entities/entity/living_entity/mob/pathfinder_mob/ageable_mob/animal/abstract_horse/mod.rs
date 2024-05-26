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

/// An enum of all options that can be enabled of a rideable animal
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HorseInfo {
    /// The horse is tamed
    IsTame = 0x02,
    /// The Horse has a saddle
    IsSaddled = 0x04,
    /// The horse has bred
    HasBred = 0x08,
    /// The horse is eating
    IsEating = 0x10,
    /// The horse is rearing
    IsRearing = 0x20,
    /// The horse has an opened mouth
    IsMouthOpen = 0x40,
}
impl Into<u8> for HorseInfo {
    fn into(self) -> u8 {
        todo!()
    }
}

/// An interface of a horse
#[derive(PartialEq, Default)]
pub struct AbstractHorse {
    animal: Animal,
    /// A mask of all options that can be toggled of a horse
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
