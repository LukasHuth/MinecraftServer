use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Zoglin {
    monster: Monster,
    pub baby: bool,
}
impl Deref for Zoglin {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Zoglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
