use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;

/// An instance of a thrown snowball
#[derive(Default)]
pub struct Snowball {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Deref for Snowball {
    type Target = ThrownItemProtectile;

    fn deref(&self) -> &Self::Target {
        &self.thrown_item_protectile
    }
}
impl DerefMut for Snowball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thrown_item_protectile
    }
}
