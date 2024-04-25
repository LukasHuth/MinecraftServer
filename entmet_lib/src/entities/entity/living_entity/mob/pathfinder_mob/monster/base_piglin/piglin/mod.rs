use std::ops::{Deref, DerefMut};

use super::BasePiglin;

/// An instance of a piglin
#[derive(Default)]
pub struct Piglin {
    base_piglin: BasePiglin,
    /// Whether it is a baby or not
    pub baby: bool,
    /// Whether it is charging its crossbow or not
    pub charging_crossbow: bool,
    /// Whether it is dancing or not
    pub dancing: bool,
}
impl Deref for Piglin {
    type Target = BasePiglin;

    fn deref(&self) -> &Self::Target {
        &self.base_piglin
    }
}
impl DerefMut for Piglin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base_piglin
    }
}
