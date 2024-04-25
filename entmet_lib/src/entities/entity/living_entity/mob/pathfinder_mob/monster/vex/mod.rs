use std::ops::{Deref, DerefMut};

use super::Monster;

/// An instance of a vex
#[derive(Default)]
pub struct Vex {
    monster: Monster,
    /// Whether it is attacking something or not
    pub attacking: bool,
}
impl Deref for Vex {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Vex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
