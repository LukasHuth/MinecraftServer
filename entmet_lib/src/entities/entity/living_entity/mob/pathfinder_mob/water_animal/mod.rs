use std::ops::{Deref, DerefMut};

use super::PathfinderMob;
pub mod squid;
pub mod dolphin;
pub mod abstract_fish;
#[derive(Default)]
pub struct WaterAnimal {
    pathfinder_mob: PathfinderMob,
}
impl Deref for WaterAnimal {
    type Target = PathfinderMob;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob
    }
}
impl DerefMut for WaterAnimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob
    }
}
