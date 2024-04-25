use std::ops::{Deref, DerefMut};

use slot_lib::Slot;
use super::Entity;

mod thrown_egg;
pub use thrown_egg::*;
mod thrown_ender_pearl;
pub use thrown_ender_pearl::*;
mod thrown_experience_bottle;
pub use thrown_experience_bottle::*;
mod thrown_potion;
pub use thrown_potion::*;
mod snowball;
pub use snowball::*;

/// An iterface for a thrown projectile
#[derive(Default)]
pub struct ThrownItemProtectile {
    entity: Entity,
    /// The item data of the thrown protectile
    pub slot: Slot,
}
impl Deref for ThrownItemProtectile {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for ThrownItemProtectile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
