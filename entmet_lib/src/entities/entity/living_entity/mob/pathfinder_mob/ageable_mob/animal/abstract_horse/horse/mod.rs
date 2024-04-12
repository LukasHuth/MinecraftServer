use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

#[derive(Default)]
pub struct Horse {
    abstract_horse: AbstractHorse,
    pub variant: i32,
}
impl Deref for Horse {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for Horse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
