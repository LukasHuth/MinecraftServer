use std::ops::{Deref, DerefMut};

use super::AbstractMinecartContainer;

/// An instance of a minecart that carries a chest
#[derive(Default)]
pub struct MinecartChest {
    abstract_minecart_container: AbstractMinecartContainer,
}
impl Deref for MinecartChest {
    type Target = AbstractMinecartContainer;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_container
    }
}
impl DerefMut for MinecartChest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_container
    }
}
