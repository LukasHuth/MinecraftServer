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
    pub wood_type: BoatWoodType,
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
    pub splash_timer: i32,
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
