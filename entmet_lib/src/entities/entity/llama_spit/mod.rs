use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of a llama spit
#[derive(Default)]
pub struct LlamaSpit {
    entity: Entity,
}
impl Deref for LlamaSpit {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for LlamaSpit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
