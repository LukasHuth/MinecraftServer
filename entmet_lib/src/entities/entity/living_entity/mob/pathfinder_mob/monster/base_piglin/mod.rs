use std::ops::{Deref, DerefMut};

use super::Monster;

mod piglin;
pub use piglin::*;
mod piglin_brute;
pub use piglin_brute::*;

#[derive(Default)]
pub struct BasePiglin {
    monster: Monster,
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
