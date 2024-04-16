use std::ops::{Deref, DerefMut};

use super::Raider;

#[derive(Default)]
pub struct Ravager {
    raider: Raider,
}
impl Deref for Ravager {
    type Target = Raider;

    fn deref(&self) -> &Self::Target {
        &self.raider
    }
}
impl DerefMut for Ravager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider
    }
}
