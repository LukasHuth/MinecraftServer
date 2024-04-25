use std::ops::{Deref, DerefMut};

use super::Entity;

/// An instance of an interaction
pub struct Interaction {
    entity: Entity,
    /// The width
    pub width: f32,
    /// The height
    pub height: f32,
    /// Whether it can be attacked/interacted with or not
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
    /// Function to create a new instance of an `Interaction`
    ///
    /// # Arguments
    /// `width` - The width of the interaction
    /// `height` - The height of the interaction
    /// `responsive` - Wether it is interactable or not
    /// `entity` - Optional `Entity`, if `None` the default entity is used
    pub fn new(width: f32, height: f32, responsive: bool, entity: Option<Entity>) -> Self {
        let entity = match entity {
            Some(v) => v,
            None => Entity::default(),
        };
        Self { entity, width, height, responsive }
    }
}
