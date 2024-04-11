use std::ops::{Deref, DerefMut};

use super::Entity;

pub struct WitherSkull {
    entity: Entity,
    pub invulnerable: bool,
}
impl Deref for WitherSkull {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for WitherSkull {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for WitherSkull {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            invulnerable: false,
        }
    }
}
