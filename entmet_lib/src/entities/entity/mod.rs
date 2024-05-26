use crate::datatypes::{Mask, PoseEnum};

mod interaction;
pub use interaction::*;
mod display;
pub use display::*;
mod thrown_item_protectile;
pub use thrown_item_protectile::*;
mod eye_of_ender;
pub use eye_of_ender::*;
mod falling_block;
pub use falling_block::*;
mod area_effect_cloud;
pub use area_effect_cloud::*;
mod fishing_hook;
pub use fishing_hook::*;
mod abstract_arrow;
pub use abstract_arrow::*;
mod abstract_vehicle;
pub use abstract_vehicle::*;
mod end_crystal;
pub use end_crystal::*;
mod dragon_fireball;
pub use dragon_fireball::*;
mod small_fireball;
pub use small_fireball::*;
mod fireball;
pub use fireball::*;
mod wither_skull;
pub use wither_skull::*;
mod firework_rocket_entity;
pub use firework_rocket_entity::*;
mod item_frame;
pub use item_frame::*;
mod painting;
pub use painting::*;
mod item_entity;
pub use item_entity::*;
mod living_entity;
pub use living_entity::*;
mod evoker_fangs;
pub use evoker_fangs::*;
mod llama_spit;
pub use llama_spit::*;
mod primed_tnt;
pub use primed_tnt::*;

/// An enum of the possible entity states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityState {
    /// The mob is on fire
    IsOnFire = 0x01,
    /// The mob is crouching
    IsCrouching = 0x02,
    /// The mob is sprinting
    IsSprinting = 0x08,
    /// The mob is swimming
    IsSwimming = 0x10,
    /// The mob is invisible
    IsInvisible = 0x20,
    /// The mob has the glowing effect
    HasGlowingEffect = 0x40,
    /// The mob is flying with an elytra
    IsFlyingWithAnElytra = 0x80,
}
impl Into<u8> for EntityState {
    fn into(self) -> u8 {
        self as u8
    }
}
const ALL_ENTITY_STATES: [EntityState; 8] = [EntityState::IsOnFire, EntityState::IsCrouching, EntityState::IsCrouching, EntityState::IsSprinting, EntityState::IsSwimming, EntityState::IsInvisible,
    EntityState::HasGlowingEffect, EntityState::IsFlyingWithAnElytra];
/// An struct that is able to hold entity states
#[derive(Clone)]
pub struct EntityStatusHolder {
    states: Vec<EntityState>,
    mask: u8,
}
impl From<Vec<EntityState>> for EntityStatusHolder {
    fn from(states: Vec<EntityState>) -> Self {
        let mut mask = 0;
        for state in states.iter() {
            mask |= *state as u8;
        }
        Self { mask, states }
    }
}
impl From<u8> for EntityStatusHolder {
    fn from(mask: u8) -> Self {
        let states = ALL_ENTITY_STATES.iter().enumerate()
            .filter(|&(i, _)| ((mask >> i as u8) & 1) != 0)
            .map(|(_, &data)|data).collect();
        Self { mask, states }
    }
}
impl EntityStatusHolder {
    /// Function to add a state to an entity
    pub fn add(&mut self, state: EntityState) {
        if (self.mask & state as u8) != 0 { return }
        self.states.push(state);
        self.mask |= state as u8;
    }
    /// Function to remove a state from an entity
    pub fn remove(&mut self, state: EntityState) {
        if (self.mask & state as u8) == 0 { return }
        self.states.retain(|&old_state| old_state != state);
        self.mask &= u8::MAX ^ state as u8;
    }
}
/// An struct for a basic Entity
#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    /// The states of the entity
    pub status: Mask<EntityState>,
    // pub status: EntityStatusHolder,
    /// The amount of ticks in the air
    pub air_ticks: i32,
    /// The optional custom name of the entity
    pub custom_name: Option<nbt_lib::datatypes::TextComponent>,
    /// Whether the custom name of the entity is visible
    pub custom_name_visible: bool,
    /// Wether the entity is silent or not
    pub silent: bool,
    /// Wether gravity applies to the entity or not
    pub no_gravity: bool,
    /// The pose of the enum
    pub pose: PoseEnum,
    /// The amount of ticks that the entity is freezing in powdered snow
    pub ticks_frozen_in_powdered_snow: i32,
}
impl Default for Entity {
    fn default() -> Self {
        Self {
            status: Mask::default(),
            air_ticks: 300,
            custom_name: None,
            custom_name_visible: false,
            silent: false,
            no_gravity: false,
            pose: PoseEnum::Standing,
            ticks_frozen_in_powdered_snow: 0,
        }
    }
}
impl Entity {
    /// Function to create a new instance of `Entity`
    ///
    /// # Arguments
    /// `status` - An instance of `Mask` holding `EntityState`, storing the states of the entity
    /// `air_ticks` - The amount of ticks in the air
    /// `custom_name` - Optional custom name of the entity
    /// `custom_name_visible` - Whether the custom name should be visible
    /// `silent` - Whether the entity is silent or not
    /// `no_gravity` - Whether gravity should apply to the entity or not
    /// `pose` - The pose of the entity
    /// `tickts_frozen_in_powdered_snow` - The amount that the player is freezing in powdered snow
    pub fn new(
        status: Mask<EntityState>,
        air_ticks: i32,
        custom_name: Option<nbt_lib::datatypes::TextComponent>,
        custom_name_visible: bool,
        silent: bool,
        no_gravity: bool,
        pose: PoseEnum,
        ticks_frozen_in_powdered_snow: i32
    ) -> Self {
        Self { status, air_ticks, custom_name_visible, custom_name, silent, no_gravity, pose, ticks_frozen_in_powdered_snow }
    }
}
