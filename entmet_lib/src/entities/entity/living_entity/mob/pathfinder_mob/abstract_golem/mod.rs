
use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

pub mod iron_golem;
pub mod snow_golem;
pub mod shulker;

#[derive(Default)]
pub struct AbstractGolem {
    pathfinder_mob: PathfinderMob,
}
impl Deref for AbstractGolem {
    type Target = PathfinderMob;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob
    }
}
impl DerefMut for AbstractGolem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob
    }
}
