use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Wither {
    monster: Monster,
    pub center_head_target: Option<i32>,
    pub left_head_target: Option<i32>,
    pub right_head_target: Option<i32>,
    pub invulnerable_time: i32,
}
impl Deref for Wither {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Wither {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
