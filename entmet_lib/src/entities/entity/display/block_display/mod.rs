use std::ops::{Deref, DerefMut};

use super::Display;

pub struct BlockDisplay {
    display: Display,
    pub block_state: i32,
}
impl Default for BlockDisplay {
    fn default() -> Self {
        Self { display: Display::default(), block_state: 0 }
    }
}
impl Deref for BlockDisplay {
    type Target = Display;

    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
impl DerefMut for BlockDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}
