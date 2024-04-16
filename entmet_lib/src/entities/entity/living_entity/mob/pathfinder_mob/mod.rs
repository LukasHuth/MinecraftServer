use std::ops::{Deref, DerefMut};

use super::Mob;

pub mod water_animal;
pub mod ageable_mob;
pub mod abstract_golem;
pub mod monster;

#[derive(Default)]
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
