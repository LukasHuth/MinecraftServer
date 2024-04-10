use slot_lib::Slot;

use super::Entity;

#[derive(Default)]
pub struct ItemEntity {
    entity: Entity,
    slot: Slot,
}
