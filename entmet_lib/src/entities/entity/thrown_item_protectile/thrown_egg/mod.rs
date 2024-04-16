use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;

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
impl Default for ThrownEgg {
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
