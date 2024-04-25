
use std::ops::{Deref, DerefMut};

use super::AbstractVillager;

/// An instance of a wandering trader
#[derive(Default)]
pub struct WanderingTrader {
    abstract_villager: AbstractVillager,
}
impl Deref for WanderingTrader {
    type Target = AbstractVillager;

    fn deref(&self) -> &Self::Target {
        &self.abstract_villager
    }
}
impl DerefMut for WanderingTrader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_villager
    }
}
