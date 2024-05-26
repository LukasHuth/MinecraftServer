use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Entity;
mod player;
pub use player::*;
mod armor_stand;
pub use armor_stand::*;
mod mob;
pub use mob::*;

/// An enum of the state of the hand of an entity
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HandStates {
    /// The hand does nothing
    #[default] None = 0,
    /// The main hand is active
    HandActiveMain = 0b01,
    /// The offhand is active
    HandActiveOffhand = 0b11,
    /// A riptide spin is active
    RiptideSpin = 0x04,
}
impl Into<u8> for HandStates {
    fn into(self) -> u8 {
        self as u8
    }
}
/// An interface if a living entity
#[derive(PartialEq, Debug, Clone)]
pub struct LivingEntity {
    entity: Entity,
    /// The state of the hands of the entity
    pub hand_state: Mask<HandStates>,
    /// The health of the entity
    pub health: f32,
    /// The color of the current potion effect
    pub potion_effect_color: i32,
    /// Wether the potion effect color is visible or not
    pub potion_effect_ambient: bool,
    /// How many arrows are stuck inside of the entity
    pub arrows_inside: i32,
    /// How many bee stings the entity has
    pub bee_stingers_inside: i32,
    /// Optional location of the bed that the entity is currently sleeping in
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
            hand_state: Mask::default(),
            health: 1.0,
            potion_effect_color: 0,
            potion_effect_ambient: false,
            arrows_inside: 0,
            bee_stingers_inside: 0,
            location_of_bed_currently_sleeping_in: None,
        }
    }
}
