use std::ops::{Deref, DerefMut};

use super::ItemFrame;

/// An instance of an item frame that is glowing
#[derive(Default)]
pub struct GlowingItemFrame {
    item_frame: ItemFrame,
}
impl Deref for GlowingItemFrame {
    type Target = ItemFrame;

    fn deref(&self) -> &Self::Target {
        &self.item_frame
    }
}
impl DerefMut for GlowingItemFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item_frame
    }
}
