use std::ops::{Deref, DerefMut};

use crate::datatypes::particles::ParticleType;

use super::Entity;

/// An instance of an area effect cloud
#[derive(PartialEq)]
pub struct AreaEffectCloud {
    entity: Entity,
    /// The radius of the effect
    pub radius: f32,
    /// The color of the effect
    pub color: i32,
    /// Whether the radius should be ignored or not
    pub ignore_radius: bool,
    /// The particle type of the area of effect
    pub particle: ParticleType,
}
impl Default for AreaEffectCloud {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            radius: 0.5,
            color: 0,
            ignore_radius: false,
            particle: ParticleType::Effect,
        }
    }
}
impl Deref for AreaEffectCloud {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for AreaEffectCloud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
