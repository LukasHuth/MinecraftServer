use std::ops::{Deref, DerefMut};

use super::Monster;

/// An instance of an endermite
#[derive(PartialEq, Default)]
pub struct Endermite {
    monster: Monster,
}
impl Deref for Endermite {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Endermite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
