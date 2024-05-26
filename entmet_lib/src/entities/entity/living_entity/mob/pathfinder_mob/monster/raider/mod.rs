use std::ops::{Deref, DerefMut};

use super::Monster;

mod abstract_illager;
pub use abstract_illager::*;
mod ravager;
pub use ravager::*;
mod witch;
pub use witch::*;

/// An interface of a raider
#[derive(PartialEq, Default)]
pub struct Raider {
    monster: Monster,
    /// Whether it is celebrating ot not
    pub celebrating: bool,
}
impl Deref for Raider {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Raider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
