use std::ops::{Deref, DerefMut};

use super::AbstractVehicle;

mod chest_boat;
pub use chest_boat::*;

/// An enum of the wood types that a boat can consist of
#[derive(PartialEq, Clone, Copy, Default)]
pub enum BoatWoodType {
    /// Oak wood
    #[default] Oak = 0,
    /// Spruce wood
    Spruce = 1,
    /// Birch wood
    Birch = 2,
    /// Jungle wood
    Jungle = 3,
    /// Acacia wood
    Acacia = 4,
    /// Dark oak wood
    DarkOak = 5,
}
/// An instance of a boat
#[derive(PartialEq, Default)]
pub struct Boat {
    abstract_vehicle: AbstractVehicle,
    /// The wood type of the boat
    pub wood_type: BoatWoodType,
    /// Whether the left paddle is turning or not
    pub left_paddle_turning: bool,
    /// Whether the right paddle is turning or not
    pub right_paddle_turning: bool,
    /// The splash timer
    pub splash_timer: i32,
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
