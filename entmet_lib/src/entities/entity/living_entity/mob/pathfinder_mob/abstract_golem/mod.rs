
use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

mod iron_golem;
pub use iron_golem::*;
mod snow_golem;
pub use snow_golem::*;
mod shulker;
pub use shulker::*;

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
