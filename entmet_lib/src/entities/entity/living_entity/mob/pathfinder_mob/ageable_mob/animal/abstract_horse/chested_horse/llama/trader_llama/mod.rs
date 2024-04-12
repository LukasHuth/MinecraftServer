use std::ops::{Deref, DerefMut};

use super::Llama;

#[derive(Default)]
pub struct TraderLlama {
    llama: Llama,
}
impl Deref for TraderLlama {
    type Target = Llama;

    fn deref(&self) -> &Self::Target {
        &self.llama
    }
}
impl DerefMut for TraderLlama {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.llama
    }
}
