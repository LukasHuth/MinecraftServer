use std::ops::{Deref, DerefMut};

use super::Zombie;

/// An instance of a drowned
#[derive(PartialEq, Default)]
pub struct Drowned {
    zombie: Zombie,
}
impl Deref for Drowned {
    type Target = Zombie;

    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for Drowned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}
