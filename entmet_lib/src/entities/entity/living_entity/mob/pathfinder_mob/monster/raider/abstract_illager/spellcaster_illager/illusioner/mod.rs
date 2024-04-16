use std::ops::{Deref, DerefMut};

use super::SpellcasterIllager;

#[derive(Default)]
pub struct Illusioner {
    spellcaster_illager: SpellcasterIllager,
}
impl Deref for Illusioner {
    type Target = SpellcasterIllager;

    fn deref(&self) -> &Self::Target {
        &self.spellcaster_illager
    }
}
impl DerefMut for Illusioner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spellcaster_illager
    }
}
