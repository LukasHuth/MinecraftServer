use std::ops::{Deref, DerefMut};

use super::Zombie;

#[derive(Default)]
pub struct Husk {
    zombie: Zombie,
}
impl Deref for Husk {
    type Target = Zombie;

    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for Husk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}
