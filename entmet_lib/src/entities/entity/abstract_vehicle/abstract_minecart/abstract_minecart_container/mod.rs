use std::ops::{Deref, DerefMut};

use super::AbstractMinecart;

mod minecart_hopper;
pub use minecart_hopper::*;
mod minecart_chest;
pub use minecart_chest::*;

/// An interface to store container minecart data
#[derive(Default)]
pub struct AbstractMinecartContainer {
    abstract_minecart: AbstractMinecart,
}
impl Deref for AbstractMinecartContainer {
    type Target = AbstractMinecart;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for AbstractMinecartContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}
