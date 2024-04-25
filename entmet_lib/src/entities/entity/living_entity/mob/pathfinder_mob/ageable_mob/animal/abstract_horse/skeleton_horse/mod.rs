use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

/// An instance of a skeleton horse
#[derive(Default)]
pub struct SkeletonHorse {
    abstract_horse: AbstractHorse,
}
impl Deref for SkeletonHorse {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for SkeletonHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
