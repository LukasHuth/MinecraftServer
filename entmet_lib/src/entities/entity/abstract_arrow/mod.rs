use super::{Entity, EntityFunctions};

pub mod spectral_arrow;
pub mod thrown_trident;

pub struct AbstractArrow {
    entity: Entity,
    color: i32,
}
pub trait AbstractArrowFunctions: EntityFunctions {
     fn get_color(&self) -> i32;
    fn set_color(&mut self, value: i32);
}
impl EntityFunctions for AbstractArrow {
    fn get_states(&self) -> &[super::EntityState] {
        self.entity.get_states()
    }
    fn add_state(&mut self, state: super::EntityState) {
        self.entity.add_state(state)
    }
    fn remove_state(&mut self, state: super::EntityState) {
        self.entity.remove_state(state)
    }
    fn get_air_ticks(&self) -> i32 {
        self.entity.get_air_ticks()
    }
    fn set_air_ticks(&mut self, value: i32) {
        self.entity.set_air_ticks(value)
    }
    fn increment_air_ticks(&mut self) {
        self.entity.increment_air_ticks()
    }
    fn get_custom_name(&self) -> Option<&nbt_lib::datatypes::TextComponent> {
        self.entity.get_custom_name()
    }
    fn set_custom_name(&mut self, value: Option<nbt_lib::datatypes::TextComponent>) {
        self.entity.set_custom_name(value)
    }
    fn is_custom_name_visible(&self) -> bool {
        self.entity.is_custom_name_visible()
    }
    fn set_custom_name_visible(&mut self, value: bool) {
        self.entity.set_custom_name_visible(value)
    }
    fn is_silent(&self) -> bool {
        self.entity.is_silent()
    }
    fn set_silent(&mut self, value: bool) {
        self.entity.set_silent(value)
    }
    fn has_no_gravity(&self) -> bool {
        self.entity.has_no_gravity()
    }
    fn set_no_gravity(&mut self, value: bool) {
        self.entity.set_no_gravity(value)
    }
    fn get_pose(&self) -> crate::datatypes::PoseEnum {
        self.entity.get_pose()
    }
    fn set_pose(&mut self, value: crate::datatypes::PoseEnum) {
        self.entity.set_pose(value)
    }
    fn get_ticks_frozen_in_powdered_snow(&self) -> i32 {
        self.entity.get_ticks_frozen_in_powdered_snow()
    }
    fn set_ticks_frozen_in_powdered_snow(&mut self, value: i32) {
        self.entity.set_ticks_frozen_in_powdered_snow(value)
    }
    fn increment_ticks_frozen_in_powdered_snow(&mut self) {
        self.entity.increment_ticks_frozen_in_powdered_snow()
    }
}
impl AbstractArrowFunctions for AbstractArrow {
    fn get_color(&self) -> i32 {
        self.color
    }
    fn set_color(&mut self, value: i32) {
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
