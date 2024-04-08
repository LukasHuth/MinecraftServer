use super::Entity;

pub struct WitherSkull {
    entity: Entity,
    invulnerable: bool,
}
impl Default for WitherSkull {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            invulnerable: false,
        }
    }
}
