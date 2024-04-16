use std::ops::{Deref, DerefMut};

use super::Monster;

pub mod skeleton;
pub mod wither_skeleton;
pub mod stray;

#[derive(Default)]
pub struct AbstractSkeleton {
    monster: Monster,
}
impl Deref for AbstractSkeleton {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for AbstractSkeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
