use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of a primmed tnt
pub struct PrimedTnt {
    entity: Entity,
    /// The time in ticks, until it explodes
    pub fuse_time: i32,
}
impl Default for PrimedTnt {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            fuse_time: 80,
        }
    }
}
impl Deref for PrimedTnt {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for PrimedTnt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
