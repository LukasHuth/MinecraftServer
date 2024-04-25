use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

/// An instance of a camel
#[derive(Default)]
pub struct Camel {
    abstract_horse: AbstractHorse,
    /// Wether it is dashing or not
    pub dashing: bool,
    /// The last tick that the camel changed their pose
    pub last_pose_change_tick: i64,
}
impl Deref for Camel {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for Camel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
