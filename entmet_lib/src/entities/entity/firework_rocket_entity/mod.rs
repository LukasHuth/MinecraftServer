use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Entity;

/// An instance of a firework rocket
#[derive(Default)]
pub struct FireworkRocketEntity {
    entity: Entity,
    /// The firework data
    pub slot: Slot,
    /// The entity that used it, if one used it
    pub user_entity_id: Option<i32>,
    /// if it is shot from an angle (crossbow)
    pub shot_at_angle: bool,
}
impl Deref for FireworkRocketEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for FireworkRocketEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
