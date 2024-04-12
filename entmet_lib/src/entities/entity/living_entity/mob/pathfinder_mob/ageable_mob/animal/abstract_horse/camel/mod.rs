use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

#[derive(Default)]
pub struct Camel {
    abstract_horse: AbstractHorse,
    pub dashing: bool,
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
