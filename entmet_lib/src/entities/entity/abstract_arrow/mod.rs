use std::ops::{Deref, DerefMut};

use super::Entity;

mod arrow;
pub use arrow::*;
mod spectral_arrow;
pub use spectral_arrow::*;
mod thrown_trident;
pub use thrown_trident::*;

/// An interface to hold data for arrows
#[derive(Default)]
pub struct AbstractArrow {
    entity: Entity,
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
