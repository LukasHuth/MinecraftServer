use super::Entity;

pub mod spectral_arrow;
pub mod thrown_trident;

pub struct AbstractArrow {
    entity: Entity,
    color: i32,
}
impl Default for AbstractArrow {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            color: -1,
        }
    }
}
