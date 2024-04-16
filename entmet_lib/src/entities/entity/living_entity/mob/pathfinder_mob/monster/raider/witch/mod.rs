use std::ops::{Deref, DerefMut};

use super::Raider;

#[derive(Default)]
pub struct Witch {
    raider: Raider,
    pub drinking_potion: bool,
}
impl Deref for Witch {
    type Target = Raider;

    fn deref(&self) -> &Self::Target {
        &self.raider
    }
}
impl DerefMut for Witch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider
    }
}
