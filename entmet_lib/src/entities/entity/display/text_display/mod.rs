use std::ops::{Deref, DerefMut};

use nbt_lib::datatypes::TextComponent;

use super::Display;

/// An enum of the different attributes that an text display can have
#[repr(u8)]
pub enum TextDisplayMaskData {
    /// Whether it has a shadow or not
    HasShadow = 0x01,
    /// Whether it is see through or not
    IsSeeThrough = 0x02,
    /// Whether to use the default background color or not
    UseDefaultBackgroundColor = 0x04,
    /// Aligned centered
    AlignCenter = 0x08,
    /// Aligned left
    AlignLeft = 0x08 + 0x01, // or + 0x03,
    /// Aligned right
    AlignRight = 0x08 + 0x02,
}
/// An instance of a text display
pub struct TextDisplay {
    display: Display,
    /// The text of the display
    pub text: TextComponent,
    /// the line width
    pub line_width: i32,
    /// The background color
    ///
    /// # Note
    ///
    /// The color is decoded as RGB / ARGB
    /// where:
    /// - B: & 0xFF
    /// - G: & 0xFF00 >> 8
    /// - R: & 0xFF0000 >> 16
    /// - A: & 0xFF000000 >> 24
    pub background_color: i32,
    /// the text opacity ranging from 0 to 255
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
