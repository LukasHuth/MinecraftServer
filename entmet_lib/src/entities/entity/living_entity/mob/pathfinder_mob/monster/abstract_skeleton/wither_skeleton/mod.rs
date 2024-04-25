use std::ops::{Deref, DerefMut};

use super::AbstractSkeleton;

/// An instance of a wither skeleton
#[derive(Default)]
pub struct WitherSkeleton {
    abstract_skeleton: AbstractSkeleton,
}
impl Deref for WitherSkeleton {
    type Target = AbstractSkeleton;

    fn deref(&self) -> &Self::Target {
        &self.abstract_skeleton
    }
}
impl DerefMut for WitherSkeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_skeleton
    }
}
