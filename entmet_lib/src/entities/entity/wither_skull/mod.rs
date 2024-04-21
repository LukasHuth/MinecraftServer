use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of a wither skull
pub struct WitherSkull {
    entity: Entity,
    /// whether it is invulnerable or not
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
