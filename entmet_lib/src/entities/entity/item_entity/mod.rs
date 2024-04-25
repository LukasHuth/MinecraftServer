use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

/// An instance of a dropped item
#[derive(Default)]
pub struct ItemEntity {
    entity: Entity,
    /// Slot data of the item
    pub slot: Slot,
}
impl Deref for ItemEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for ItemEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
