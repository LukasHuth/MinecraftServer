use std::ops::{Deref, DerefMut};

use super::AbstractIllager;

#[derive(Default)]
pub struct Pillager {
    abstract_illager: AbstractIllager,
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
