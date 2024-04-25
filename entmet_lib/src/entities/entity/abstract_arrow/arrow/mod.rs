use std::ops::{Deref, DerefMut};

use minecraft_assets::color::Color;

use super::AbstractArrow;

/// An instance of a minecraft arrow or tipped arrow
#[derive(Default)]
pub struct Arrow {
    abstract_arrow: AbstractArrow,
    /// The color of the arrow
    ///
    /// # Note
    ///
    /// The color should be set to `None` if it is an arrow and the color should be set for a
    /// tipped arrow
    pub color: Option<Color>,
}
impl Deref for Arrow {
    type Target = AbstractArrow;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow
    }
}
impl DerefMut for Arrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow
    }
}
