use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

/// An instance of a zombie horse
#[derive(Default)]
pub struct ZombieHorse {
    abstract_horse: AbstractHorse,
}
impl Deref for ZombieHorse {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for ZombieHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
