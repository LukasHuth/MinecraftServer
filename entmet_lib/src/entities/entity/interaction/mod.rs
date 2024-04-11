use std::ops::{Deref, DerefMut};

use super::Entity;

pub struct Interaction {
    entity: Entity,
    pub width: f32,
    pub height: f32,
    pub responsive: bool,
}
impl Deref for Interaction {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for Interaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for Interaction {
    fn default() -> Self {
        let entity = Entity::default();
        let width = 1.0;
        let height = 1.0;
        let responsive = false;
        Self { entity, width, height, responsive }
    }
}
impl Interaction {
    pub fn new(width: f32, height: f32, responsive: bool, entity: Option<Entity>) -> Self {
        let entity = match entity {
            Some(v) => v,
            None => Entity::default(),
        };
        Self { entity, width, height, responsive }
    }
}
