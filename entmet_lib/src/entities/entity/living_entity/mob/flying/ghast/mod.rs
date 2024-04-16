use std::ops::{Deref, DerefMut};

use super::Flying;

#[derive(Default)]
pub struct Ghast {
    flying: Flying,
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
