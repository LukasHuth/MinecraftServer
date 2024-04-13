use std::ops::{Deref, DerefMut};

use super::AbstractGolem;

#[derive(Default)]
pub struct SnowGolem {
    abstract_golem: AbstractGolem,
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
