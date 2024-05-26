use std::ops::{Deref, DerefMut};

use super::AgeableMob;

mod sniffer;
pub use sniffer::*;
mod abstract_horse;
pub use abstract_horse::*;
mod axolotl;
pub use axolotl::*;
mod bee;
pub use bee::*;
mod fox;
pub use fox::*;
mod frog;
pub use frog::*;
mod ocelot;
pub use ocelot::*;
mod panda;
pub use panda::*;
mod pig;
pub use pig::*;
mod rabbit;
pub use rabbit::*;
mod turtle;
pub use turtle::*;
mod polar_bear;
pub use polar_bear::*;
mod chicken;
pub use chicken::*;
mod cow;
pub use cow::*;
mod hoglin;
pub use hoglin::*;
mod sheep;
pub use sheep::*;
mod strider;
pub use strider::*;
mod tameable_animal;
pub use tameable_animal::*;
mod goat;
pub use goat::*;

/// An interface of an animal
#[derive(PartialEq, Debug, Default, Clone)]
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
