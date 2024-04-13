use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

#[derive(Default)]
pub struct Monster {
    pathfinder_mob: PathfinderMob,
}
impl Deref for Monster {
    type Target = PathfinderMob;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob
    }
}
impl DerefMut for Monster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob
    }
}
