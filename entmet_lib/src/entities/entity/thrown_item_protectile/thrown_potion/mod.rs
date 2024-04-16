use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;


pub struct ThrownPotion {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Deref for ThrownPotion {
    type Target = ThrownItemProtectile;

    fn deref(&self) -> &Self::Target {
        &self.thrown_item_protectile
    }
}
impl DerefMut for ThrownPotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thrown_item_protectile
    }
}
impl Default for ThrownPotion {
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
