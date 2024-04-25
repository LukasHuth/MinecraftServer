use std::ops::{Deref, DerefMut};

use super::Animal;

/// An instance of a goat
pub struct Goat {
    animal: Animal,
    /// Whether it is screaming or not
    pub screaming: bool,
    /// Whether it has its left horn or not
    pub left_horn: bool,
    /// Whether it has its right horn or not
    pub right_horn: bool,
}
impl Default for Goat {
    fn default() -> Self {
        Self {
            animal: Animal::default(),
            screaming: false,
            left_horn: true,
            right_horn: true,
        }
    }
}
impl Deref for Goat {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Goat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
