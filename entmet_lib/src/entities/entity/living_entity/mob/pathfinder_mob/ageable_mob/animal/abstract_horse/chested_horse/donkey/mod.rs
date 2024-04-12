use std::ops::{Deref, DerefMut};

use super::ChestedHorse;

#[derive(Default)]
pub struct Donkey {
    chested_horse: ChestedHorse,
    pub chest: bool,
}
impl Deref for Donkey {
    type Target = ChestedHorse;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse
    }
}
impl DerefMut for Donkey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse
    }
}
