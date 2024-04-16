use std::ops::{Deref, DerefMut};

use super::ChestedHorse;

pub mod trader_llama;

#[derive(Default)]
#[repr(u8)]
pub enum LlamaVariant {
    #[default] Creamy = 0,
    White = 1,
    Brown = 2,
    Gray = 3,
}
#[derive(Default)]
pub struct Llama {
    chested_horse: ChestedHorse,
    pub strength: i32,
    pub carpet_color: i32,
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
