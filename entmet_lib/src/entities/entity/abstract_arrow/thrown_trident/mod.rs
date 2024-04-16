use std::ops::{Deref, DerefMut};

use super::AbstractArrow;

pub struct ThrownTrident {
    abstract_arrow: AbstractArrow,
    loyality_level: i32,
    has_enchantment_glint: bool,
}
impl Default for ThrownTrident {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            loyality_level: 0,
            has_enchantment_glint: false,
        }
    }
}
impl Deref for ThrownTrident {
    type Target = AbstractArrow;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow
    }
}
impl DerefMut for ThrownTrident {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow
    }
}
impl ThrownTrident {
    pub fn get_loyality_level(&self) -> i32 {
        self.loyality_level
    }
    pub fn set_loyality_level(&mut self, value: i32) {
        self.loyality_level = value;
    }
    pub fn has_enchantment_glint(&self) -> bool {
        self.has_enchantment_glint
    }
    pub fn set_has_enchantment_glint(&mut self, value: bool) {
        self.has_enchantment_glint = value;
    }
}
