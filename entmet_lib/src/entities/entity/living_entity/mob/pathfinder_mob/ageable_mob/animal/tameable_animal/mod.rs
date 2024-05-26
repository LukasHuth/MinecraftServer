use std::ops::{Deref, DerefMut};

use super::Animal;

mod cat;
pub use cat::*;
mod wolf;
pub use wolf::*;
mod parrot;
pub use parrot::*;

/// An interface of a tameable amimal
#[derive(PartialEq, Default)]
pub struct TameableAnimal {
    animal: Animal,
    /// Whether it is sitting or not
    pub sitting: bool,
    /// Whether it is sitting or not
    pub tamed: bool,
    /// The uuid of the owner, if it has one
    pub owner: Option<u128>,
}
impl Deref for TameableAnimal {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for TameableAnimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
