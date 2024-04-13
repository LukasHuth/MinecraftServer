use std::ops::{Deref, DerefMut};

use crate::datatypes::{Direction, MinecraftColor};

use super::AbstractGolem;

#[derive(Default)]
#[repr(u8)]
pub enum SkulkerColor {
    Mc(MinecraftColor),
    #[default] None = 16,
}

#[derive(Default)]
pub struct Shulker {
    abstract_golem: AbstractGolem,
    pub attach_face: Direction,
    pub shield_height: i8,
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
