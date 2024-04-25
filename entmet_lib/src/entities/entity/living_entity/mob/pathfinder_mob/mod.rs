use std::ops::{Deref, DerefMut};

use super::Mob;

mod water_animal;
pub use water_animal::*;
mod ageable_mob;
pub use ageable_mob::*;
mod abstract_golem;
pub use abstract_golem::*;
mod monster;
pub use monster::*;

/// An interface for mobs using pathfinder
#[derive(Default, Clone)]
pub struct PathfinderMob {
    mob: Mob,
}
impl Deref for PathfinderMob {
    type Target = Mob;

    fn deref(&self) -> &Self::Target {
        &self.mob
    }
}
impl DerefMut for PathfinderMob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob
    }
}
