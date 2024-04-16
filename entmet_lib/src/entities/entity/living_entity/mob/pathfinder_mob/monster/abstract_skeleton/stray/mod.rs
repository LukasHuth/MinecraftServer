use std::ops::{Deref, DerefMut};

use super::AbstractSkeleton;

#[derive(Default)]
pub struct Stray {
    abstract_skeleton: AbstractSkeleton,
}
impl Deref for Stray {
    type Target = AbstractSkeleton;

    fn deref(&self) -> &Self::Target {
        &self.abstract_skeleton
    }
}
impl DerefMut for Stray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_skeleton
    }
}
