use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Goat {
    animal: Animal,
    pub screaming: bool,
    pub left_horn: bool,
    pub right_horn: bool,
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
