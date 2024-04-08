use super::Entity;


pub struct FallingBlock {
    entity: Entity,
    position: (i32, i32, i16),
}
impl Default for FallingBlock {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            position: (0,0,0),
        }
    }
}
