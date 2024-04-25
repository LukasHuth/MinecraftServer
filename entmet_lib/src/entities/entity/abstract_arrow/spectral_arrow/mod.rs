use std::ops::{Deref, DerefMut};

use super::AbstractArrow;

/// An instance of a Spectral Arrow
#[derive(Default)]
pub struct SpectralArrow {
    abstract_arrow: AbstractArrow,
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
