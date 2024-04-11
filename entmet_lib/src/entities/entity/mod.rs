use crate::datatypes::PoseEnum;

pub mod interaction;
pub mod display;
pub mod thrown_item_protectile;
pub mod eye_of_ender;
pub mod falling_block;
pub mod area_effect_cloud;
pub mod fishing_hook;
pub mod abstract_arrow;
pub mod abstract_vehicle;
pub mod end_crystal;
pub mod dragon_fireball;
pub mod small_fireball;
pub mod fireball;
pub mod wither_skull;
pub mod firework_rocket_entity;
pub mod item_frame;
pub mod painting;
pub mod item_entity;
pub mod living_entity;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityState {
    IsOnFire = 0x01,
    IsCrouching = 0x02,
    IsSprinting = 0x08,
    IsSwimming = 0x10,
    IsInvisible = 0x20,
    HasGlowingEffect = 0x40,
    IsFlyingWithAnElytra = 0x80,
}
const ALL_ENTITY_STATES: [EntityState; 8] = [EntityState::IsOnFire, EntityState::IsCrouching, EntityState::IsCrouching, EntityState::IsSprinting, EntityState::IsSwimming, EntityState::IsInvisible,
    EntityState::HasGlowingEffect, EntityState::IsFlyingWithAnElytra];
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
    pub fn add(&mut self, state: EntityState) {
        if (self.mask & state as u8) != 0 { return }
        self.states.push(state);
        self.mask |= state as u8;
    }
    pub fn remove(&mut self, state: EntityState) {
        if (self.mask & state as u8) == 0 { return }
        self.states.retain(|&old_state| old_state != state);
        self.mask &= u8::MAX ^ state as u8;
    }
}
pub struct Entity {
    status: EntityStatusHolder,
    air_ticks: i32,
    custom_name: Option<nbt_lib::datatypes::TextComponent>,
    custom_name_visible: bool,
    silent: bool,
    no_gravity: bool,
    pose: PoseEnum,
    ticks_frozen_in_powdered_snow: i32,
}
impl Default for Entity {
    fn default() -> Self {
        Self {
            status: EntityStatusHolder::from(vec![]),
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
    pub fn get_states(&self) -> &[EntityState] {
        self.status.states.as_slice()
    }

    pub fn add_state(&mut self, state: EntityState) {
        self.status.add(state)
    }

    pub fn remove_state(&mut self, state: EntityState) {
        self.status.remove(state)
    }

    pub fn get_air_ticks(&self) -> i32 {
        self.air_ticks
    }

    pub fn set_air_ticks(&mut self, value: i32) {
        self.air_ticks = value;
    }

    pub fn increment_air_ticks(&mut self) {
        self.air_ticks += 1;
    }

    pub fn get_custom_name(&self) -> Option<&nbt_lib::datatypes::TextComponent> {
        self.custom_name.as_ref()
    }

    pub fn set_custom_name(&mut self, value: Option<nbt_lib::datatypes::TextComponent>) {
        self.custom_name = value;
    }

    pub fn is_custom_name_visible(&self) -> bool {
        self.custom_name_visible
    }

    pub fn set_custom_name_visible(&mut self, value: bool) {
        self.custom_name_visible = value;
    }

    pub fn is_silent(&self) -> bool {
        self.silent
    }

    pub fn set_silent(&mut self, value: bool) {
        self.silent = value;
    }

    pub fn has_no_gravity(&self) -> bool {
        self.no_gravity
    }

    pub fn set_no_gravity(&mut self, value: bool) {
        self.no_gravity = value;
    }

    pub fn get_pose(&self) -> PoseEnum {
        self.pose
    }

    pub fn set_pose(&mut self, value: PoseEnum) {
        self.pose = value;
    }

    pub fn get_ticks_frozen_in_powdered_snow(&self) -> i32 {
        self.ticks_frozen_in_powdered_snow
    }

    pub fn set_ticks_frozen_in_powdered_snow(&mut self, value: i32) {
        self.ticks_frozen_in_powdered_snow = value;
    }

    pub fn increment_ticks_frozen_in_powdered_snow(&mut self) {
        self.ticks_frozen_in_powdered_snow += 1;
    }
}
impl Entity {
    pub fn new(
        status: EntityStatusHolder,
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

