use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum SheepColorVariant {
    #[default] White = 0x00,
    Orange = 0x01,
    Magenta = 0x02,
    LightBlue = 0x03,
    Yellow = 0x04,
    Lime = 0x05,
    Pink = 0x06,
    Gray = 0x07,
    LightGray = 0x08,
    Cyan = 0x09,
    Purple = 0x0A,
    Blue = 0x0B,
    Brown = 0x0C,
    Green = 0x0D,
    Red = 0x0E,
    Black = 0x0F,
}

#[derive(Default)]
pub struct Sheep {
    animal: Animal,
    pub variant: SheepColorVariant,
    pub sheard: bool,
}
impl Deref for Sheep {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Sheep {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
