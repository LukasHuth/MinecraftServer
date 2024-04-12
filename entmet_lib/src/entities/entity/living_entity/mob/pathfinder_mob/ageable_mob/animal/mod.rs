use std::ops::{Deref, DerefMut};

use super::AgeableMob;

pub mod sniffer;
pub mod abstract_horse;
pub mod axolotl;
pub mod bee;
pub mod fox;
pub mod frog;
pub mod ocelot;
pub mod panda;
pub mod pig;
pub mod rabbit;
pub mod turtle;
pub mod polar_bear;
pub mod chicken;
pub mod cow;
pub mod hoglin;
pub mod sheep;
pub mod strider;
pub mod tameable_animal;

#[derive(Default)]
pub struct Animal {
    ageable_mob: AgeableMob,
}
impl Deref for Animal {
    type Target = AgeableMob;

    fn deref(&self) -> &Self::Target {
        &self.ageable_mob
    }
}
impl DerefMut for Animal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_mob
    }
}
