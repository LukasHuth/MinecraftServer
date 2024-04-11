use std::ops::{Deref, DerefMut};

use super::Entity;

pub mod boat;

pub struct AbstractVehicle {
    entity: Entity,
    pub shaking_power: i32,
    pub shaking_direction: i32,
    pub shaking_multiplier: f32,
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
impl Deref for AbstractVehicle {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for AbstractVehicle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
