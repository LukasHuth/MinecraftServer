use std::ops::{Deref, DerefMut};

use super::Entity;

pub mod arrow;
pub mod spectral_arrow;
pub mod thrown_trident;

pub struct AbstractArrow {
    entity: Entity,
    color: i32,
}
impl AbstractArrow {
    pub fn get_color(&self) -> i32 {
        self.color
    }
    pub fn set_color(&mut self, value: i32) {
        self.color = value;
    }
}
impl Default for AbstractArrow {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            color: -1,
        }
    }
}
impl Deref for AbstractArrow {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for AbstractArrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
