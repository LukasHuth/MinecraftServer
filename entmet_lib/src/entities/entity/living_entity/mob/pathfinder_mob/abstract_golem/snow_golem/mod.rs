use std::ops::{Deref, DerefMut};

use super::AbstractGolem;

/// An instance of a snow golem (snow man)
#[derive(Default)]
pub struct SnowGolem {
    abstract_golem: AbstractGolem,
    /// Whether it lost it's pumpkin or not
    ///
    /// # Note
    ///  Default is `false` because it has it's head by default
    pub no_pumpkin_hat: bool,
}
impl Deref for SnowGolem {
    type Target = AbstractGolem;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem
    }
}
impl DerefMut for SnowGolem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem
    }
}
