use std::ops::{Deref, DerefMut};

use minecraft_assets::color::Color;

use crate::datatypes::Direction;

use super::AbstractGolem;

/// An enum for the shulker color
#[derive(Default)]
#[repr(u8)]
pub enum SkulkerColor {
    /// One of the dye colors
    Mc(Color),
    /// The default color
    #[default] None = 16,
}

/// An instance of a shulker
#[derive(Default)]
pub struct Shulker {
    abstract_golem: AbstractGolem,
    /// The direction, that the shulker is attached to
    pub attach_face: Direction,
    /// The height of it's shield (upper part)
    pub shield_height: i8,
    /// The color of the shulker
    pub color: SkulkerColor,
}
impl Deref for Shulker {
    type Target = AbstractGolem;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem
    }
}
impl DerefMut for Shulker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem
    }
}
