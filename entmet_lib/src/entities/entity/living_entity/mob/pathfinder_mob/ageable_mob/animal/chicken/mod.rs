use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Chicken {
    animal: Animal,
}
impl Deref for Chicken {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Chicken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
