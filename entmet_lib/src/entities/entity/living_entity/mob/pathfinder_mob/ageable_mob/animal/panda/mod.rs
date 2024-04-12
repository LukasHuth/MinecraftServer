use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum PandaGene {
    #[default] Normal = 0,
    Lazy = 1,
    Worried = 2,
    Playful = 3,
    Brown = 4,
    Weak = 5,
    Aggressive = 6,
}

#[repr(u8)]
pub enum PandaInfo {
    IsSneezing = 0x02,
    IsRolling = 0x04,
    IsSitting = 0x08,
    IsOnBack = 0x10,
}

#[derive(Default)]
pub struct Panda {
    animal: Animal,
    pub breed_timer: i32,
    pub sneeze_timer: i32,
    pub eat_timer: i32,
    pub main_gene: PandaGene,
    pub hidden_gene: PandaGene,
    pub info: Vec<PandaInfo>,
}
impl Deref for Panda {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Panda {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
