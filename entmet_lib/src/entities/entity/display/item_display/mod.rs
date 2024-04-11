use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Display;

#[repr(u8)]
pub enum DisplayType {
    None = 0,
    ThirdPersonLeftHand = 1,
    ThirdPersonRightHand = 2,
    FirstPersonLeftHand = 3,
    FirstPersonRightHand = 4,
    Head = 5,
    Gui = 6,
    Ground = 7,
    Fixed = 8,
}
pub struct ItemDisplay {
    display: Display,
    pub slot: Slot,
    pub display_type: DisplayType,
}
impl Deref for ItemDisplay {
    type Target = Display;

    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
impl DerefMut for ItemDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}
impl Default for ItemDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            slot: Slot::Empty,
            display_type: DisplayType::None,
        }
    }
}
