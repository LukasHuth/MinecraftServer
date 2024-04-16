use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

pub mod glowing_item_frame;

pub struct ItemFrame {
    entity: Entity,
    pub slot: Slot,
    pub rotation: i32,
}
impl Deref for ItemFrame {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for ItemFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for ItemFrame {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
            rotation: 0,
        }
    }
}
