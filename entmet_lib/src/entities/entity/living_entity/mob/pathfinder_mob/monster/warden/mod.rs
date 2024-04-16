use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Warden {
    monster: Monster,
    pub anger_level: i32,
}
impl Deref for Warden {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Warden {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
