//! This module contains data for colors in minecraft

/// An enum of Minecraft colors
///
/// # Sources
/// - [minecraft.fandom.com/wiki/Tropical_Fish](https://minecraft.fandom.com/wiki/Tropical_Fish#Color)
/// - [minecraft.fandom.com/wiki/Cat](https://minecraft.fandom.com/wiki/Cat#Collar_color)
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Default, Clone, Copy)]
pub enum Color {
    /// White #F7FFFE
    White = 0,
    /// Orange #F9801D
    Orange = 1,
    /// Magenta #C74EBD
    Magenta = 2,
    /// Light Blue #3AB3DA
    LightBlue = 3,
    /// Yellow #FED83D
    Yellow = 4,
    /// Lime #80C71F
    Lime = 5,
    /// Pink #F38BAA
    Pink = 6,
    /// Gray #474F52
    Gray = 7,
    /// Light Gray #9D9D97
    LightGray = 8,
    /// Cyan #169C9C
    Cyan = 9,
    /// Purple #8932B8
    Purple = 10,
    /// Blue #3C44AA
    Blue = 11,
    /// Brown #835432
    Brown = 12,
    /// Green #5E7C16
    Green = 13,
    /// Red #B02E26
    #[default] Red = 14,
    /// Black #1D1D21
    Black = 15,
}
