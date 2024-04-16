use std::ops::{Deref, DerefMut};

use super::Entity;

pub struct FishingHook {
    entity: Entity,
    pub hooked_entity: Option<i32>,
    pub is_catchable: bool,
}
impl Deref for FishingHook {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for FishingHook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for FishingHook {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            hooked_entity: None,
            is_catchable: false,
        }
    }
}
