use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

#[derive(Default)]
pub struct SmallFireball {
    entity: Entity,
    pub slot: Slot,
}
impl Deref for SmallFireball {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for SmallFireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
