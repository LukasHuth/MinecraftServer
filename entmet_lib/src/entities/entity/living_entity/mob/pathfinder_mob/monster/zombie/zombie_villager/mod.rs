use std::ops::{Deref, DerefMut};

use crate::datatypes::VillagerData;

use super::Zombie;

/// An instance of a zombie villager
#[derive(Default)]
pub struct ZombieVillager {
    zombie: Zombie,
    /// Whether it is converting or not
    pub converting: bool,
    /// The villager data of the underlying villager
    pub villager_data: VillagerData,
}
impl Deref for ZombieVillager {
    type Target = Zombie;

    fn deref(&self) -> &Self::Target {
        &self.zombie
    }
}
impl DerefMut for ZombieVillager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie
    }
}
