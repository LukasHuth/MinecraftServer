use std::ops::{Deref, DerefMut};

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
impl AbstractVehicle {
    pub fn get_shaking_power(&self) -> i32 {
        self.shaking_power
    }
    pub fn set_shaking_power(&mut self, value: i32) {
        self.shaking_power = value;
    }
    pub fn get_shaking_direction(&self) -> i32 {
        self.shaking_direction
    }
    pub fn set_shaking_direction(&mut self, value: i32) {
        self.shaking_direction = value;
    }
    pub fn get_shaking_multiplier(&self) -> f32 {
        self.shaking_multiplier
    }
    pub fn set_shaking_multiplier(&mut self, value: f32) {
        self.shaking_multiplier = value;
    }
}
