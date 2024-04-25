use std::ops::{Deref, DerefMut};

use super::AgeableMob;

mod villager;
pub use villager::*;
mod wandering_trader;
pub use wandering_trader::*;

/// An interface of a villager
#[derive(Default)]
pub struct AbstractVillager {
    ageable_mob: AgeableMob,
    /// A timer of ticks, how long the headshake is still active
    ///
    /// The timer starts at 40 and decrements each tick. While the value is 0 the villager won't
    /// shake its head
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
