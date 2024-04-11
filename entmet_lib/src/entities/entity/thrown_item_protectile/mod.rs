use std::ops::{Deref, DerefMut};

use slot_lib::Slot;
use super::Entity;

pub mod thrown_egg;
pub mod thrown_ender_pearl;
pub mod thrown_experience_bottle;
pub mod thrown_potion;
pub mod snowball;

pub struct ThrownItemProtectile {
    entity: Entity,
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
impl Default for ThrownItemProtectile {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
        }
    }
}
