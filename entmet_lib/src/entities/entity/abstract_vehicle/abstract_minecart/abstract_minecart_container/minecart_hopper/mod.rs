use std::ops::{Deref, DerefMut};

use super::AbstractMinecartContainer;

#[derive(Default)]
pub struct MinecartHopper {
    abstract_minecart_container: AbstractMinecartContainer,
}
impl Deref for MinecartHopper {
    type Target = AbstractMinecartContainer;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_container
    }
}
impl DerefMut for MinecartHopper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_container
    }
}
