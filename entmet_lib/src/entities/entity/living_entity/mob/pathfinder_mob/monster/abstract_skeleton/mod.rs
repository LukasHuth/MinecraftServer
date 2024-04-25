use std::ops::{Deref, DerefMut};

use super::Monster;

mod skeleton;
pub use skeleton::*;
mod wither_skeleton;
pub use wither_skeleton::*;
mod stray;
pub use stray::*;

/// An interface if a skeleton
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
