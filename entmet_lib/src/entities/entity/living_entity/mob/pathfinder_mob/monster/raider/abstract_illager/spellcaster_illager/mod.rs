use std::ops::{Deref, DerefMut};

use super::AbstractIllager;

mod evoker;
pub use evoker::*;
mod illusioner;
pub use illusioner::*;

/// An enum of all castable spells
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Default)]
pub enum Spell {
    #[default] None = 0,
    SummonVex = 1,
    Attack = 2,
    Wololo = 3,
    Disapear = 4,
    Blindness = 5,
}

/// An interface of a spellcasting illager
#[derive(Default)]
pub struct SpellcasterIllager {
    abstract_illager: AbstractIllager,
    /// The spell that is being casted
    pub spell: Spell,
}
impl Deref for SpellcasterIllager {
    type Target = AbstractIllager;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager
    }
}
impl DerefMut for SpellcasterIllager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager
    }
}
