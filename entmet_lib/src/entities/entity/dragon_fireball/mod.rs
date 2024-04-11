use std::ops::{Deref, DerefMut};

use super::Entity;

#[derive(Default)]
pub struct DragonFireball {
    entity: Entity,
}
impl Deref for DragonFireball {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for DragonFireball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
