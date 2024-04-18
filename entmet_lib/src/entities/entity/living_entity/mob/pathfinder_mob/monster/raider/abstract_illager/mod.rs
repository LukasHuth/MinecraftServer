use std::ops::{Deref, DerefMut};

use super::Raider;

mod vindicator;
pub use vindicator::*;
mod pillager;
pub use pillager::*;
mod spellcaster_illager;
pub use spellcaster_illager::*;

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
