use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

/// An instance of a fireball
#[derive(Default)]
pub struct Fireball {
    entity: Entity,
    /// The item of the fireball
    pub slot: Slot,
}
impl Deref for Fireball {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for Fireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
