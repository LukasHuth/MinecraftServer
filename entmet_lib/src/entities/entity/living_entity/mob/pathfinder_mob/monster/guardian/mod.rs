use std::ops::{Deref, DerefMut};

use super::Monster;

mod elder_guardian;
pub use elder_guardian::*;

/// An instance of a Guardian
#[derive(Default)]
pub struct Guardian {
    monster: Monster,
    /// Whether it is retracting its spikes or not
    pub restracting_spikes: bool,
    /// The id of its target
    pub target_eid: Option<i32>,
}
impl Deref for Guardian {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Guardian {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
