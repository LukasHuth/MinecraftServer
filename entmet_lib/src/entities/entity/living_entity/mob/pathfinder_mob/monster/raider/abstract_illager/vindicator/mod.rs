use std::ops::{Deref, DerefMut};

use super::AbstractIllager;

#[derive(Default)]
pub struct Vindicator {
    abstract_illager: AbstractIllager
}
impl Deref for Vindicator {
    type Target = AbstractIllager;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager
    }
}
impl DerefMut for Vindicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager
    }
}
