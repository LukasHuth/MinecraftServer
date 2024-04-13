use std::ops::{Deref, DerefMut};

use super::AbstractMinecart;

#[derive(Default)]
pub struct AbstractMinecartHolder {
    abstract_minecart: AbstractMinecart,
}
impl Deref for AbstractMinecartHolder {
    type Target = AbstractMinecart;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart
    }
}
impl DerefMut for AbstractMinecartHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart
    }
}
