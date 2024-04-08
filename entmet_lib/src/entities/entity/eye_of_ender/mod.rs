use slot_lib::Slot;

use super::Entity;

pub struct EyeOfEnder {
    entity: Entity,
    slot: Slot,
}
impl Default for EyeOfEnder {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
        }
    }
}
