use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;

/// An instance of a thrown egg
#[derive(Default)]
pub struct ThrownEgg {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Deref for ThrownEgg {
    type Target = ThrownItemProtectile;

    fn deref(&self) -> &Self::Target {
        &self.thrown_item_protectile
    }
}
impl DerefMut for ThrownEgg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thrown_item_protectile
    }
}
