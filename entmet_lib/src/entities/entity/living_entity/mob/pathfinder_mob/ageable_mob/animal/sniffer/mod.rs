use std::ops::{Deref, DerefMut};

use crate::datatypes::SnifferEnum;

use super::Animal;

#[derive(Default)]
pub struct Sniffer {
    animal: Animal,
    pub state: SnifferEnum,
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
