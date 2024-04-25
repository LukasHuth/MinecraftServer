use std::ops::{Deref, DerefMut};

use nbt_lib::datatypes::TextComponent;

use super::AbstractMinecart;

/// An instance of a minecart that carries a command block
#[derive(Default)]
pub struct MinecartCommandBlock {
    abstract_minecart: AbstractMinecart,
    /// The command of the command block
    pub command: String,
    /// The last output of the command block
    pub last_output: TextComponent,
}
impl Deref for MinecartCommandBlock {
    type Target = AbstractMinecart;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for MinecartCommandBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}
