use std::ops::{Deref, DerefMut};

use super::Monster;

mod zombie_villager;
pub use zombie_villager::*;
mod husk;
pub use husk::*;
mod drowned;
pub use drowned::*;
mod zombified_piglin;
pub use zombified_piglin::*;

/// An instance of a zombie
///
/// # Note
///
/// Also used for inheritance of all other zombiefied monsters
#[derive(PartialEq, Default)]
pub struct Zombie {
    monster: Monster,
    /// Whether it is a baby or not
    pub baby: bool,
    /// Whether it is becomming a drowned or not
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
