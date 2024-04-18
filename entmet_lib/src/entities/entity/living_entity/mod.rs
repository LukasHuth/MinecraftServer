use std::ops::{Deref, DerefMut};

use super::Entity;
mod player;
pub use player::*;
mod armor_stand;
pub use armor_stand::*;
mod mob;
pub use mob::*;
#[derive(Clone)]
#[repr(u8)]
pub enum HandStates {
    None = 0,
    HandActiveMain = 0b01,
    HandActiveOffhand = 0b11,
    RiptideSpin = 0x04,
}
#[derive(Clone)]
pub struct LivingEntity {
    entity: Entity,
    pub hand_state: HandStates,
    pub health: f32,
    pub potion_effect_color: i32,
    pub potion_effect_ambient: bool,
    pub arrows_inside: i32,
    pub bee_stingers_inside: i32,
    pub location_of_bed_currently_sleeping_in: Option<(i32, i32, i32)>,
}
impl Deref for LivingEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for LivingEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for LivingEntity {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            hand_state: HandStates::None,
            health: 1.0,
            potion_effect_color: 0,
            potion_effect_ambient: false,
            arrows_inside: 0,
            bee_stingers_inside: 0,
            location_of_bed_currently_sleeping_in: None,
        }
    }
}
