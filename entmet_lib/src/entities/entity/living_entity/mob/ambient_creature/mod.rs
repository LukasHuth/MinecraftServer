use std::ops::{Deref, DerefMut};

use super::Mob;
mod bat;
pub use bat::*;

/// An interface of an ambient creature
#[derive(Default)]
pub struct AmbientCreature {
    mob: Mob,
}
impl Deref for AmbientCreature {
    type Target = Mob;

    fn deref(&self) -> &Self::Target {
        &self.mob
    }
}
impl DerefMut for AmbientCreature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob
    }
}
