use std::ops::{Deref, DerefMut};

use super::Mob;

pub struct Slime {
    mob: Mob,
    pub size: i32,
}
impl Default for Slime {
    fn default() -> Self {
        Self {
            mob: Mob::default(),
            size: 1,
        }
    }
}
impl Deref for Slime {
    type Target = Mob;

    fn deref(&self) -> &Self::Target {
        &self.mob
    }
}
impl DerefMut for Slime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob
    }
}
