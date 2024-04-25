use std::ops::{Deref, DerefMut};

use super::Flying;

/// An instance of a ghost
#[derive(Default)]
pub struct Ghast {
    flying: Flying,
    /// Whether it is attacking something or not
    pub attacking: bool,
}
impl Deref for Ghast {
    type Target = Flying;

    fn deref(&self) -> &Self::Target {
        &self.flying
    }
}
impl DerefMut for Ghast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flying
    }
}
