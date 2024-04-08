use slot_lib::Slot;
use super::Entity;

pub mod thrown_egg;
pub mod thrown_ender_pearl;
pub mod thrown_experience_bottle;
pub mod thrown_potion;
pub mod snowball;

pub struct ThrownItemProtectile {
    entity: Entity,
    slot: Slot,
}
impl Default for ThrownItemProtectile {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
        }
    }
}
