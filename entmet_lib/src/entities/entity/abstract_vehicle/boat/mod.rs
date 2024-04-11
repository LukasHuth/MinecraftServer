use std::ops::{Deref, DerefMut};

use super::AbstractVehicle;

pub mod chest_boat;

#[derive(Clone, Copy)]
pub enum BoatWoodType {
    Oak = 0,
    Spruce = 1,
    Birch = 2,
    Jungle = 3,
    Acacia = 4,
    DarkOak = 5,
}
pub struct Boat {
    abstract_vehicle: AbstractVehicle,
    wood_type: BoatWoodType,
    left_paddle_turning: bool,
    right_paddle_turning: bool,
    splash_timer: i32,
}
impl Default for Boat {
    fn default() -> Self {
        Self {
            abstract_vehicle: AbstractVehicle::default(),
            wood_type: BoatWoodType::Oak,
            left_paddle_turning: false,
            right_paddle_turning: false,
            splash_timer: 0,
        }
    }
}
impl Deref for Boat {
    type Target = AbstractVehicle;

    fn deref(&self) -> &Self::Target {
        &self.abstract_vehicle
    }
}
impl DerefMut for Boat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_vehicle
    }
}
impl Boat {
    pub fn get_wood_type(&self) -> BoatWoodType {
        self.wood_type
    }
    pub fn set_wood_type(&mut self, value: BoatWoodType) {
        self.wood_type = value;
    }
    pub fn is_left_paddle_turning(&self) -> bool {
        self.left_paddle_turning
    }
    pub fn has_left_paddle_turning(&mut self, value: bool) {
        self.left_paddle_turning = value;
    }
    pub fn is_right_paddle_turning(&self) -> bool {
        self.right_paddle_turning
    }
    pub fn has_right_paddle_turning(&mut self, value: bool) {
        self.right_paddle_turning = value;
    }
    pub fn get_splash_timer(&self) -> i32{
        self.splash_timer
    }
    pub fn set_splash_timer(&mut self, value: i32) {
        self.splash_timer = value;
    }
}
