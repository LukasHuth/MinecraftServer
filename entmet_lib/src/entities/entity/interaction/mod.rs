use super::Entity;

pub struct Interaction {
    entity: Entity,
    width: f32,
    height: f32,
    responsive: bool,
}
impl Default for Interaction {
    fn default() -> Self {
        let entity = Entity::default();
        let width = 1.0;
        let height = 1.0;
        let responsive = false;
        Self { entity, width, height, responsive }
    }
}
impl Interaction {
    pub fn new(width: f32, height: f32, responsive: bool, entity: Option<Entity>) -> Self {
        let entity = match entity {
            Some(v) => v,
            None => Entity::default(),
        };
        Self { entity, width, height, responsive }
    }
}
