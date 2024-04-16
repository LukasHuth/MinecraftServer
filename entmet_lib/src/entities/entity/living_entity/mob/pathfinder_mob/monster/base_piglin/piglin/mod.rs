use std::ops::{Deref, DerefMut};

use super::BasePiglin;

#[derive(Default)]
pub struct Piglin {
    base_piglin: BasePiglin,
    pub baby: bool,
    pub charging_crossbow: bool,
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
