use std::ops::{Deref, DerefMut};

use super::ChestedHorse;

#[derive(Default)]
pub struct Mule {
    chested_horse: ChestedHorse,
}
impl Deref for Mule {
    type Target = ChestedHorse;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse
    }
}
impl DerefMut for Mule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse
    }
}
