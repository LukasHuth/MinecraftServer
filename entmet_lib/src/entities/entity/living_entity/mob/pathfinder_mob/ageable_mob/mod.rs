use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

mod animal;
pub use animal::*;
mod abstract_villager;
pub use abstract_villager::*;

/// An interface of a mab that can age
#[derive(Default, Clone)]
pub struct AgeableMob {
    pathfinder_mob: PathfinderMob,
    /// Whether it is a baby or not
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
