use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

#[derive(Default)]
pub struct ItemEntity {
    entity: Entity,
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
