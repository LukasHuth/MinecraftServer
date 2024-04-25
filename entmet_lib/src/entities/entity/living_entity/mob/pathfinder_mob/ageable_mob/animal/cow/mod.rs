use std::ops::{Deref, DerefMut};

use super::Animal;

mod mooshroom;
pub use mooshroom::*;

/// An instance of a Cow
#[derive(Default, Clone)]
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
