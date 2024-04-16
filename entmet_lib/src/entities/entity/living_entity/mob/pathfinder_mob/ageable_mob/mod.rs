use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

pub mod animal;
pub mod abstract_villager;

#[derive(Default)]
pub struct AgeableMob {
    pathfinder_mob: PathfinderMob,
    pub is_baby: bool,
}
impl Deref for AgeableMob {
    type Target = PathfinderMob;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob
    }
}
impl DerefMut for AgeableMob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob
    }
}
