use std::ops::{Deref, DerefMut};

use slot_lib::Slot;

use super::Display;

/// An enum of the different available display types
#[repr(u8)]
pub enum DisplayType {
    /// No display type
    None = 0,
    /// In third person in the left hand
    ThirdPersonLeftHand = 1,
    /// In third person in the right hand
    ThirdPersonRightHand = 2,
    /// In first person in the left hand
    FirstPersonLeftHand = 3,
    /// In first person in the right hand
    FirstPersonRightHand = 4,
    /// An the head
    Head = 5,
    /// In the gui
    Gui = 6,
    /// At the ground
    Ground = 7,
    /// Fixed at one place
    Fixed = 8,
}
/// An instance of an item display
pub struct ItemDisplay {
    display: Display,
    /// Slot data of the displayed item
    pub slot: Slot,
    /// display type
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
