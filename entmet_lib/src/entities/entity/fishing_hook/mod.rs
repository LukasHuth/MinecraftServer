use crate::entities::entity_types::EntityEnum as EntityEnum;

use super::Entity;


pub struct FishingHook {
    entity: Entity,
    hooked_entity: Option<EntityEnum>,
    is_catchable: bool,
}
impl Default for FishingHook {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            hooked_entity: None,
            is_catchable: false,
        }
    }
}
