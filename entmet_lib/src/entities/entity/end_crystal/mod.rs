use super::Entity;

pub struct EndCrystal {
    entity: Entity,
    beam_target: Option<(i32, i32, i16)>,
    show_bottom: bool,
}
impl Default for EndCrystal {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            beam_target: None,
            show_bottom: true,
        }
    }
}
