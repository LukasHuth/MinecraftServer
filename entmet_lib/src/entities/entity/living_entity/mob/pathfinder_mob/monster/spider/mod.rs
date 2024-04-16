use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Spider {
    monster: Monster,
    pub climbing: bool,
}
impl Deref for Spider {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Spider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
