use std::ops::{Deref, DerefMut};

use crate::datatypes::MinecraftColor;

use super::AbstractArrow;

#[derive(Default)]
pub struct Arrow {
    abstract_arrow: AbstractArrow,
    pub color: Option<MinecraftColor>,
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
