use slot_lib::Slot;

use super::Entity;

pub struct ItemFrame {
    entity: Entity,
    slot: Slot,
    rotation: i32,
}
impl Default for ItemFrame {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
            rotation: 0,
        }
    }
}
