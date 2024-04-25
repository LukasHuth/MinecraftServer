use std::ops::{Deref, DerefMut};

use minecraft_assets::color::Color;

use super::ChestedHorse;

mod trader_llama;
pub use trader_llama::*;

/// An enum of all llama variants
#[derive(Default)]
#[repr(u8)]
pub enum LlamaVariant {
    /// The creamy default variant
    #[default] Creamy = 0,
    /// The white variant
    White = 1,
    /// The brown variant
    Brown = 2,
    /// The gray variant
    Gray = 3,
}
/// An instance of a llama
#[derive(Default)]
pub struct Llama {
    chested_horse: ChestedHorse,
    /// The strength of the llama
    pub strength: i32,
    /// The color of the carpet, `None` (-1) if it has no carpet
    pub carpet_color: Option<Color>,
    /// The variant of the llama
    pub variant: LlamaVariant,
}
impl Deref for Llama {
    type Target = ChestedHorse;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse
    }
}
impl DerefMut for Llama {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse
    }
}
