use std::ops::{Deref, DerefMut};

use super::Raider;

pub mod vindicator;
pub mod pillager;
pub mod spellcaster_illager;

#[derive(Default)]
pub struct AbstractIllager {
    raider: Raider,
}
impl Deref for AbstractIllager {
    type Target = Raider;

    fn deref(&self) -> &Self::Target {
        &self.raider
    }
}
impl DerefMut for AbstractIllager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider
    }
}
