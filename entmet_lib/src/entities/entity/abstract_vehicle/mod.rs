use super::Entity;

pub mod boat;

pub struct AbstractVehicle {
    entity: Entity,
    shaking_power: i32,
    shaking_direction: i32,
    shaking_multiplier: f32,
}
impl Default for AbstractVehicle {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            shaking_power: 0,
            shaking_direction: 1,
            shaking_multiplier: 0.0,
        }
    }
}
