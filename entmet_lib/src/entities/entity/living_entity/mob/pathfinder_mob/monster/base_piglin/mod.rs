use std::ops::{Deref, DerefMut};

use super::Monster;

mod piglin;
pub use piglin::*;
mod piglin_brute;
pub use piglin_brute::*;

/// An interface of a piglin
#[derive(Default)]
pub struct BasePiglin {
    monster: Monster,
    /// Whether it is immune to zombification or not
    pub immune_to_zombification: bool,
}
impl Deref for BasePiglin {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for BasePiglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
