use std::ops::{Deref, DerefMut};

use super::Animal;

pub mod mooshroom;

#[derive(Default)]
pub struct Cow {
    animal: Animal,
}
impl Deref for Cow {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Cow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
