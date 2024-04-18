use std::ops::{Deref, DerefMut};

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

#[derive(Clone)]
#[repr(u8)]
pub enum MobInfo {
    NoAI = 0x01,
    LeftHanded = 0x02,
    Aggressive = 0x04,
}
#[derive(Clone)]
pub struct Mob {
    living_entity: LivingEntity,
    pub info: Vec<MobInfo>,
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
impl Default for Mob {
    fn default() -> Self {
        Self {
            living_entity: LivingEntity::default(),
            info: Vec::new(),
        }
    }
}
