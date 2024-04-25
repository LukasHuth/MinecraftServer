use std::ops::{Deref, DerefMut};

use super::AbstractSkeleton;

/// An instance of a skeleton
#[derive(Default)]
pub struct Skeleton {
    abstract_skeleton: AbstractSkeleton,
}
impl Deref for Skeleton {
    type Target = AbstractSkeleton;

    fn deref(&self) -> &Self::Target {
        &self.abstract_skeleton
    }
}
impl DerefMut for Skeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_skeleton
    }
}
