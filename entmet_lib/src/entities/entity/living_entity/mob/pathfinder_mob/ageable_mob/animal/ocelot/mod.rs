use std::ops::{Deref, DerefMut};

use super::Animal;

/// An instance of an ocelot
#[derive(Default)]
pub struct Ocelot {
    animal: Animal,
    /// Whether it trusts the player or not
    pub trusting: bool,
}
impl Deref for Ocelot {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Ocelot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
