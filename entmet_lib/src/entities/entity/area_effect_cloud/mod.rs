use crate::datatypes::particles::ParticleType;

use super::Entity;

pub struct AreaEffectCloud {
    entity: Entity,
    radius: f32,
    color: i32,
    ignore_radius: bool,
    particle: ParticleType,
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
