use std::ops::{Deref, DerefMut};

use super::Monster;

/// An instance of a zoglin
#[derive(Default)]
pub struct Zoglin {
    monster: Monster,
    /// Whether it is a baby or not
    pub baby: bool,
}
impl Deref for Zoglin {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Zoglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
