use std::ops::{Deref, DerefMut};

use super::SpellcasterIllager;

/// An instance of an evoker
#[derive(PartialEq, Default)]
pub struct Evoker {
    spellcaster_illager: SpellcasterIllager,
}
impl Deref for Evoker {
    type Target = SpellcasterIllager;

    fn deref(&self) -> &Self::Target {
        &self.spellcaster_illager
    }
}
impl DerefMut for Evoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spellcaster_illager
    }
}
