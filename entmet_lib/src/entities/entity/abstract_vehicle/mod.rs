use std::ops::{Deref, DerefMut};

use super::Entity;

mod boat;
pub use boat::*;
mod abstract_minecart;
pub use abstract_minecart::*;

/// An Interface to store vehicle data
pub struct AbstractVehicle {
    entity: Entity,
    /// The amount of shaking of the vehicle
    pub shaking_power: i32,
    /// The direction of the shaking
    pub shaking_direction: i32,
    /// The multiplier of the shaking
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
