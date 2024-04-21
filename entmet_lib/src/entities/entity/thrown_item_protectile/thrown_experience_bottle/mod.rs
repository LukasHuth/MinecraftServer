use std::ops::{Deref, DerefMut};

use super::ThrownItemProtectile;

/// An instance of a thrown experience bottle
#[derive(Default)]
pub struct ThrownExperienceBottle {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Deref for ThrownExperienceBottle {
    type Target = ThrownItemProtectile;

    fn deref(&self) -> &Self::Target {
        &self.thrown_item_protectile
    }
}
impl DerefMut for ThrownExperienceBottle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thrown_item_protectile
    }
}
