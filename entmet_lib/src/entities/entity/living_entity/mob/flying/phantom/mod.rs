use std::ops::{Deref, DerefMut};

use super::Flying;

/// An instance of a Phantom
#[derive(Default)]
pub struct Phantom {
    flying: Flying,
    /// The size of the phantom
    pub size: i32,
}
impl Deref for Phantom {
    type Target = Flying;

    fn deref(&self) -> &Self::Target {
        &self.flying
    }
}
impl DerefMut for Phantom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flying
    }
}
