use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

mod glowing_item_frame;
pub use glowing_item_frame::*;

/// An instance of an item frame
#[derive(Default)]
pub struct ItemFrame {
    entity: Entity,
    /// The item data of the displayed item
    pub slot: Slot,
    /// The rotation of the displayed item
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
