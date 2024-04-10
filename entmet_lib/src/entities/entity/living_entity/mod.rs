use super::Entity;
pub mod player;
#[repr(u8)]
pub enum HandStates {
    None = 0,
    HandActiveMain = 0b01,
    HandActiveOffhand = 0b11,
    RiptideSpin = 0x04,
}
pub struct LivingEntity {
    entity: Entity,
    hand_state: HandStates,
    health: f32,
    potion_effect_color: i32,
    potion_effect_ambient: bool,
    arrows_inside: i32,
    bee_stingers_inside: i32,
    location_of_bed_currently_sleeping_in: Option<(i32, i32, i32)>,
}
impl Default for LivingEntity {
    fn default() -> Self {
        todo!()
    }
}
