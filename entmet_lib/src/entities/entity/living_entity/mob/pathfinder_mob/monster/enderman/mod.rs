use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Enderman {
    monster: Monster,
    pub carried_block: Option<i32>,
    pub screaming: bool,
    pub staring: bool,
}
impl Deref for Enderman {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Enderman {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
