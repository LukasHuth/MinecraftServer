use std::ops::{Deref, DerefMut};

use super::Monster;

/// An instance of an enderman
#[derive(PartialEq, Default)]
pub struct Enderman {
    monster: Monster,
    /// The id of a carried block if a block is beeing carried
    pub carried_block: Option<i32>,
    /// Whether it is screaming or not
    pub screaming: bool,
    /// Whether is is staring or not
    pub staring: bool,
}
impl Deref for Enderman {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Enderman {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
