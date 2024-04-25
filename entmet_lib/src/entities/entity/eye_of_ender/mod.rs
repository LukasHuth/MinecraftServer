use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

/// An instance of an eye of ender
#[derive(Default)]
pub struct EyeOfEnder {
    entity: Entity,
    /// an instance of the slot of the eye of ender
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
