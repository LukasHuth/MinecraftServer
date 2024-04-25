use std::ops::{Deref, DerefMut};

use super::BasePiglin;

/// An instance of a piglin brute
#[derive(Default)]
pub struct PiglinBrute {
    base_piglin: BasePiglin,
}
impl Deref for PiglinBrute {
    type Target = BasePiglin;

    fn deref(&self) -> &Self::Target {
        &self.base_piglin
    }
}
impl DerefMut for PiglinBrute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base_piglin
    }
}
