use std::ops::{Deref, DerefMut};

use super::AbstractGolem;

/// An instance of an iron golem
#[derive(Default)]
pub struct IronGolem {
    abstract_golem: AbstractGolem,
    /// Whether the iron golem was created by a player or not
    pub player_created: bool,
}
impl Deref for IronGolem {
    type Target = AbstractGolem;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem
    }
}
impl DerefMut for IronGolem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem
    }
}
