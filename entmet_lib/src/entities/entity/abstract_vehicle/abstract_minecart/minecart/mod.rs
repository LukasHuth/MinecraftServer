use std::ops::{Deref, DerefMut};

use super::AbstractMinecart;

/// An instance of a minecart
#[derive(Default)]
pub struct Minecart {
    abstract_minecart: AbstractMinecart,
}
impl Deref for Minecart {
    type Target = AbstractMinecart;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for Minecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}
