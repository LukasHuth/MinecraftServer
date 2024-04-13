use std::ops::{Deref, DerefMut};

use super::AgeableMob;

pub mod villager;
pub mod wandering_trader;

#[derive(Default)]
pub struct AbstractVillager {
    ageable_mob: AgeableMob,
    pub headshake_timer: i32,
}
impl Deref for AbstractVillager {
    type Target = AgeableMob;

    fn deref(&self) -> &Self::Target {
        &self.ageable_mob
    }
}
impl DerefMut for AbstractVillager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_mob
    }
}
