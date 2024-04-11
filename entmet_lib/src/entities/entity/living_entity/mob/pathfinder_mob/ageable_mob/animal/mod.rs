use std::ops::{Deref, DerefMut};

use super::AgeableMob;

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
