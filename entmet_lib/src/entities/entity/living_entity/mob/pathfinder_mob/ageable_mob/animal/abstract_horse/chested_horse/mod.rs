use std::ops::{Deref, DerefMut};

use super::AbstractHorse;

mod donkey;
pub use donkey::*;
mod llama;
pub use llama::*;
mod mule;
pub use mule::*;

#[derive(Default)]
pub struct ChestedHorse {
    abstract_horse: AbstractHorse,
    pub chest: bool,
}
impl Deref for ChestedHorse {
    type Target = AbstractHorse;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse
    }
}
impl DerefMut for ChestedHorse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse
    }
}
