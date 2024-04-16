use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Ocelot {
    animal: Animal,
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
