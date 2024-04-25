use std::ops::{Deref, DerefMut};

use crate::datatypes::SnifferEnum;

use super::Animal;

/// An instance of a sniffer
#[derive(Default)]
pub struct Sniffer {
    animal: Animal,
    /// The state of the sniffer
    pub state: SnifferEnum,
    /// The seed used for the random generation of drops for this sniffer
    pub drop_seed: i32,
}
impl Deref for Sniffer {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Sniffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
