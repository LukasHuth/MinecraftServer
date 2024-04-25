use std::ops::{Deref, DerefMut};

use super::Boat;

/// An instance of a boat carrying a chest
#[derive(Default)]
pub struct ChestBoat {
    boat: Boat,
}
impl Deref for ChestBoat {
    type Target = Boat;

    fn deref(&self) -> &Self::Target {
        &self.boat
    }
}
impl DerefMut for ChestBoat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boat
    }
}
