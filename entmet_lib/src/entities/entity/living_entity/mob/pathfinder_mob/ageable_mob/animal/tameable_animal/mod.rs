use std::ops::{Deref, DerefMut};

use super::Animal;

mod cat;
pub use cat::*;
mod wolf;
pub use wolf::*;
mod parrot;
pub use parrot::*;

#[derive(Default)]
pub struct TameableAnimal {
    animal: Animal,
    pub sitting: bool,
    pub tamed: bool,
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
