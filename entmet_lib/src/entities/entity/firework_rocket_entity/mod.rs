use slot_lib::Slot;

use crate::entities::entity_types::EntityEnum as EntityEnum;

use super::Entity;

pub struct FireworkRocketEntity {
    entity: Entity,
    slot: Slot,
    user_entity_id: Option<EntityEnum>,
    show_at_angle: bool,
}
impl Default for FireworkRocketEntity {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            slot: Slot::Empty,
            user_entity_id: None,
            show_at_angle: false,
        }
    }
}
