use std::ops::{Deref, DerefMut};

use super::Mob;
pub mod bat;
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
