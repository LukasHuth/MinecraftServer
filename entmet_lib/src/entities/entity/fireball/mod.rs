use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

#[derive(Default)]
pub struct Fireball {
    entity: Entity,
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
