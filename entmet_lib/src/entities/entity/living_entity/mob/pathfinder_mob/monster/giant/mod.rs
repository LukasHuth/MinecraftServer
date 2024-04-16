use std::ops::{Deref, DerefMut};

use super::Monster;

#[derive(Default)]
pub struct Giant {
    monster: Monster,
}
impl Deref for Giant {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Giant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
