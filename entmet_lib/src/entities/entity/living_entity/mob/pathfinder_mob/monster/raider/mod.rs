use std::ops::{Deref, DerefMut};

use super::Monster;

pub mod abstract_illager;
pub mod ravager;
pub mod witch;

#[derive(Default)]
pub struct Raider {
    monster: Monster,
    pub celebrating: bool,
}
impl Deref for Raider {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Raider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
