use std::ops::{Deref, DerefMut};

use super::PathfinderMob;
mod squid;
pub use squid::*;
mod dolphin;
pub use dolphin::*;
mod abstract_fish;
pub use abstract_fish::*;

/// An interface of a water animal
#[derive(PartialEq, Default)]
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
