use std::ops::{Deref, DerefMut};

use crate::datatypes::VillagerData;

use super::AbstractVillager;

/// An instance of a villager
#[derive(Default)]
pub struct Villager {
    abstract_villager: AbstractVillager,
    /// Specific villager data
    ///
    /// This data contains their `profession`, `variant` and `level`
    pub data: VillagerData,
}
impl Deref for Villager {
    type Target = AbstractVillager;

    fn deref(&self) -> &Self::Target {
        &self.abstract_villager
    }
}
impl DerefMut for Villager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_villager
    }
}
