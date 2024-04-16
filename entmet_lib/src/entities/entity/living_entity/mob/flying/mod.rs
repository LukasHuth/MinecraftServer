use std::ops::{Deref, DerefMut};

use super::Mob;

pub mod ghast;
pub mod phantom;

#[derive(Default)]
pub struct Flying {
    mob: Mob,
}
impl Deref for Flying {
    type Target = Mob;

    fn deref(&self) -> &Self::Target {
        &self.mob
    }
}
impl DerefMut for Flying {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob
    }
}
