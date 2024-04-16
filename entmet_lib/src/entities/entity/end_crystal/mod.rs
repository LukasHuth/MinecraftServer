use std::ops::{Deref, DerefMut};

use super::Entity;

pub struct EndCrystal {
    entity: Entity,
    pub beam_target: Option<(i32, i32, i16)>,
    pub show_bottom: bool,
}
impl Deref for EndCrystal {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for EndCrystal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
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
