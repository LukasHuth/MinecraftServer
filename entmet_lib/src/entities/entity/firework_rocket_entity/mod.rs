use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use crate::entities::entity_types::EntityEnum as EntityEnum;

use super::Entity;

pub struct FireworkRocketEntity {
    entity: Entity,
    pub slot: Slot,
    pub user_entity_id: Option<EntityEnum>,
    pub show_at_angle: bool,
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
impl Default for FireworkRocketEntity {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
            user_entity_id: None,
            show_at_angle: false,
        }
    }
}
