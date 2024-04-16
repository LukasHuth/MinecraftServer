use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

pub mod base_piglin;
pub mod blaze;
pub mod creeper;
pub mod endermite;
pub mod giant;
pub mod raider;
pub mod vex;
pub mod abstract_skeleton;
pub mod spider;
pub mod warden;
pub mod wither;
pub mod zoglin;
pub mod zombie;
pub mod enderman;
pub mod guardian;

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
