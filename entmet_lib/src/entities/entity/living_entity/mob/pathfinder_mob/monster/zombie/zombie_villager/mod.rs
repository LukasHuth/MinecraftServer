use std::ops::{Deref, DerefMut};

use crate::datatypes::VillagerData;

use super::Zombie;

#[derive(Default)]
pub struct ZombieVillager {
    zombie: Zombie,
    pub converting: bool,
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
