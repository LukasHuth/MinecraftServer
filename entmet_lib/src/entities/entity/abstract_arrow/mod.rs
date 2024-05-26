use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Entity;

mod arrow;
pub use arrow::*;
mod spectral_arrow;
pub use spectral_arrow::*;
mod thrown_trident;
pub use thrown_trident::*;

/// An enum of the Arrow data
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArrowInfo {
    /// Whether it is a critical hit or not
    IsCritical = 0x01,
    /// Whether it has no clip
    ///
    /// # Note
    ///
    /// Used for loyality tridents when they return
    IsNoclip = 0x02,
}
impl Into<u8> for ArrowInfo {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An interface to hold data for arrows
#[derive(PartialEq, Default)]
pub struct AbstractArrow {
    entity: Entity,
    /// A `Mask` holding potential data of the `AbstractArrow`
    pub info: Mask<ArrowInfo>,
}
impl Deref for AbstractArrow {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for AbstractArrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
