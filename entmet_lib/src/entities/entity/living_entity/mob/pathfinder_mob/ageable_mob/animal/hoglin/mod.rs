use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Hoglin {
    animal: Animal,
    pub immune_to_zombification: bool,
}
impl Deref for Hoglin {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Hoglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
