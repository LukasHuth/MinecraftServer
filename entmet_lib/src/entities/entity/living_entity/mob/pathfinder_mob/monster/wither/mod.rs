use std::ops::{Deref, DerefMut};

use super::Monster;

/// An instance of a wither
#[derive(Default)]
pub struct Wither {
    monster: Monster,
    /// Target id of the center header
    pub center_head_target: Option<i32>,
    /// Target id of the left header
    pub left_head_target: Option<i32>,
    /// Target id of the right header
    pub right_head_target: Option<i32>,
    /// The amount of ticks that the wither is still invulnerable
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
