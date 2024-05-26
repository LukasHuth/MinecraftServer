use std::ops::{Deref, DerefMut};

use crate::datatypes::BlockStateIdentifier;

use super::Display;

/// An instance of an item display
#[derive(PartialEq, Default)]
pub struct BlockDisplay<T: ?Sized + BlockStateIdentifier> {
    display: Display,
    /// The 
    pub block_state: T,
}
impl<T: BlockStateIdentifier> Deref for BlockDisplay<T> {
    type Target = Display;

    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
impl<T: BlockStateIdentifier> DerefMut for BlockDisplay<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}
