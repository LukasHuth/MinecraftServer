use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::LivingEntity;

mod ambient_creature;
pub use ambient_creature::*;
mod pathfinder_mob;
pub use pathfinder_mob::*;
mod ender_dragon;
pub use ender_dragon::*;
mod flying;
pub use flying::*;
mod slime;
pub use slime::*;

/// An enum of mob options
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MobInfo {
    /// Active if the mob should have no AI
    NoAI = 0x01,
    /// Active if the mob is lefthanded
    LeftHanded = 0x02,
    /// Active if the mob is aggressive
    Aggressive = 0x04,
}
impl Into<u8> for MobInfo {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An interface of a mob
#[derive(Clone, Default)]
pub struct Mob {
    living_entity: LivingEntity,
    /// A list of the options that can be active on a mob
    pub info: Mask<MobInfo>,
}
impl Deref for Mob {
    type Target = LivingEntity;

    fn deref(&self) -> &Self::Target {
        &self.living_entity
    }
}
impl DerefMut for Mob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity
    }
}
