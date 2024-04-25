use std::ops::{Deref, DerefMut};

use super::PathfinderMob;

mod base_piglin;
pub use base_piglin::*;
mod blaze;
pub use blaze::*;
mod creeper;
pub use creeper::*;
mod endermite;
pub use endermite::*;
mod giant;
pub use giant::*;
mod raider;
pub use raider::*;
mod vex;
pub use vex::*;
mod abstract_skeleton;
pub use abstract_skeleton::*;
mod spider;
pub use spider::*;
mod warden;
pub use warden::*;
mod wither;
pub use wither::*;
mod zoglin;
pub use zoglin::*;
mod zombie;
pub use zombie::*;
mod enderman;
pub use enderman::*;
mod guardian;
pub use guardian::*;

/// An interface of a monster
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
