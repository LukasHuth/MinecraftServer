use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

pub struct EyeOfEnder {
    entity: Entity,
    pub slot: Slot,
}
impl Deref for EyeOfEnder {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for EyeOfEnder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for EyeOfEnder {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
        }
    }
}
