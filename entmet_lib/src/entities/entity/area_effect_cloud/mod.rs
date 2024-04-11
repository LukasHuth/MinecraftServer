use std::ops::{Deref, DerefMut};

use crate::datatypes::particles::ParticleType;

use super::Entity;

pub struct AreaEffectCloud {
    entity: Entity,
    pub radius: f32,
    pub color: i32,
    pub ignore_radius: bool,
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
