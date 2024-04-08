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
    slot: Slot,
    display_type: DisplayType,
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
