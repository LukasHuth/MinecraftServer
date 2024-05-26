use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;

/// An instance of a thrown ender pearl
#[derive(PartialEq, Default)]
pub struct ThrownEnderPearl {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Deref for ThrownEnderPearl {
    type Target = ThrownItemProtectile;

    fn deref(&self) -> &Self::Target {
        &self.thrown_item_protectile
    }
}
impl DerefMut for ThrownEnderPearl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thrown_item_protectile
    }
}
