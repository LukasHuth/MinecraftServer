use std::ops::{Deref, DerefMut};

use super::Monster;

pub mod zombie_villager;
pub mod husk;
pub mod drowned;
pub mod zombified_piglin;

#[derive(Default)]
pub struct Zombie {
    monster: Monster,
    pub baby: bool,
    pub becoming_a_drowned: bool,
}
impl Deref for Zombie {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Zombie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
