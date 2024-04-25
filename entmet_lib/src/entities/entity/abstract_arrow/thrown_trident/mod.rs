use std::ops::{Deref, DerefMut};

use super::AbstractArrow;

/// An instance of a thrown Trident
pub struct ThrownTrident {
    abstract_arrow: AbstractArrow,
    /// The loyality level of the trident
    pub loyality_level: i32,
    /// Whether the trident has an enchantment glint or not
    pub has_enchantment_glint: bool,
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
