use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Polarbear {
    animal: Animal,
    pub standing_up: bool,
}
impl Deref for Polarbear {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Polarbear {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
