use std::ops::{Deref, DerefMut};

use super::AbstractMinecart;

/// An instance of a minecart that carries a furnace
#[derive(Default)]
pub struct MinecartFurnace {
    abstract_minecart: AbstractMinecart,
    /// Whether the minecart is fueled
    pub fuel: bool,
}
impl Deref for MinecartFurnace {
    type Target = AbstractMinecart;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for MinecartFurnace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}
