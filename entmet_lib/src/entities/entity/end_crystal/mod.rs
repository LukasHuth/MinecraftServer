use std::ops::{Deref, DerefMut};

use datatypes::Position;

use super::Entity;

/// An instance of an end crystal
#[derive(PartialEq)]
pub struct EndCrystal {
    entity: Entity,
    /// The target of the beam, if it is targetting something
    pub beam_target: Option<Position>,
    /// Whether the bottom should be visible or not
    pub show_bottom: bool,
}
impl Deref for EndCrystal {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for EndCrystal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for EndCrystal {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            beam_target: None,
            show_bottom: true,
        }
    }
}
