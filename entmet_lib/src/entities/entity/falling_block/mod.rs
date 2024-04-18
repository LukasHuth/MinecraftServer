use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of a falling block
pub struct FallingBlock {
    entity: Entity,
    /// Position of the falling block
    pub position: (i32, i32, i16),
}
impl Deref for FallingBlock {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for FallingBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for FallingBlock {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            position: (0,0,0),
        }
    }
}
