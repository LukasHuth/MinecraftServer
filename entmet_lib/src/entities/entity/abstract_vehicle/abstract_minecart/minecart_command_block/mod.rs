use std::ops::{Deref, DerefMut};

use nbt_lib::datatypes::TextComponent;

use super::AbstractMinecart;

#[derive(Default)]
pub struct MinecartCommandBlock {
    abstract_minecart: AbstractMinecart,
    pub command: String,
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
