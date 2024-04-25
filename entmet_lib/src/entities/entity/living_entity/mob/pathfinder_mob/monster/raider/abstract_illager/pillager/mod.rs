use std::ops::{Deref, DerefMut};

use super::AbstractIllager;

/// An instance of a pillager
#[derive(Default)]
pub struct Pillager {
    abstract_illager: AbstractIllager,
    /// Whether it is charging or not
    pub charging: bool,
}
impl Deref for Pillager {
    type Target = AbstractIllager;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager
    }
}
impl DerefMut for Pillager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager
    }
}
