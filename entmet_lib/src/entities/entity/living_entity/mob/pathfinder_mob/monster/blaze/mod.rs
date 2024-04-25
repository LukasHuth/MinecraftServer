use std::ops::{Deref, DerefMut};

use super::Monster;

/// An intance of a blaze
#[derive(Default)]
pub struct Blaze {
    monster: Monster,
    /// Whether it is on fire or not
    pub on_fire: bool,
}
impl Deref for Blaze {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Blaze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
