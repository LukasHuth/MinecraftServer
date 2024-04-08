use super::AbstractVehicle;

pub mod chest_boat;

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
