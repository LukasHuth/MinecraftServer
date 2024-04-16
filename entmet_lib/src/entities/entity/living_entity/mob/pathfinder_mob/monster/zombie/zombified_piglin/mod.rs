use std::ops::{Deref, DerefMut};

use super::Zombie;

#[derive(Default)]
pub struct ZombifiedPiglin {
    zombie: Zombie,
}
impl Deref for ZombifiedPiglin {
    type Target = Zombie;

    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for ZombifiedPiglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}
