use std::ops::{Deref, DerefMut};

use nbt_lib::datatypes::TextComponent;

use super::Display;

#[repr(u8)]
pub enum TextDisplayMaskData {
    HasShadow = 0x01,
    IsSeeThrough = 0x02,
    UseDefaultBackgroundColor = 0x04,
    AlignCenter = 0x08,
    AlignLeft = 0x08 + 0x01, // or + 0x03,
    AlignRight = 0x08 + 0x02,
}
pub struct TextDisplay {
    display: Display,
    pub text: TextComponent,
    pub line_width: i32,
    pub background_color: i32,
    pub text_opacity: i8,
    mask_dat: u8, // TODO: find good way for this
}
impl Deref for TextDisplay {
    type Target = Display;

    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
impl DerefMut for TextDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}
impl Default for TextDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            text: TextComponent::from("".to_string()),
            line_width: 200,
            background_color: 0x40000000,
            text_opacity: -1,
            mask_dat: 0
        }
    }
}
