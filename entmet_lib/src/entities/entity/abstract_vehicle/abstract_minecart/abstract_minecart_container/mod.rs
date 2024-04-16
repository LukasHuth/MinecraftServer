use std::ops::{Deref, DerefMut};

use super::AbstractMinecart;

pub mod minecart_hopper;
pub mod minecart_chest;

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
