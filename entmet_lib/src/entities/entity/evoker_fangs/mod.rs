use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of evoker fangs
#[derive(Default)]
pub struct EvokerFangs {
    entity: Entity,
}
impl Deref for EvokerFangs {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for EvokerFangs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
