use std::ops::{Deref, DerefMut};

use super::AbstractIllager;

pub mod evoker;
pub mod illusioner;

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

#[derive(Default)]
pub struct SpellcasterIllager {
    abstract_illager: AbstractIllager,
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
