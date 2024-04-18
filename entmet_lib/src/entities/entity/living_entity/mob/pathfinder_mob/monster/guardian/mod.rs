use std::ops::{Deref, DerefMut};

use super::Monster;

mod elder_guardian;
pub use elder_guardian::*;

#[derive(Default)]
pub struct Guardian {
    monster: Monster,
    pub restracting_spikes: bool,
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
