use std::ops::{Deref, DerefMut};

use super::AbstractArrow;

pub struct SpectralArrow {
    abstract_arrow: AbstractArrow,
}
impl Default for SpectralArrow {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
        }
    }
}
impl Deref for SpectralArrow {
    type Target = AbstractArrow;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow
    }
}
impl DerefMut for SpectralArrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow
    }
}
