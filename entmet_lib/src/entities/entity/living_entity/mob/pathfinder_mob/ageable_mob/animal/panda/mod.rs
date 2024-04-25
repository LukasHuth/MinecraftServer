use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Animal;

/// An enum of all gene variants
#[allow(missing_docs)]
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

/// An enum of all states a panda can be in
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PandaInfo {
    IsSneezing = 0x02,
    IsRolling = 0x04,
    IsSitting = 0x08,
    IsOnBack = 0x10,
}
impl Into<u8> for PandaInfo {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An instance of a panda
#[derive(Default)]
pub struct Panda {
    animal: Animal,
    /// the current breed timer
    pub breed_timer: i32,
    /// The timer until the sneeze ends
    pub sneeze_timer: i32,
    /// The timer until the eat animation ends
    pub eat_timer: i32,
    /// The main gene of the Panda
    pub main_gene: PandaGene,
    /// The hidden gene of the panda
    pub hidden_gene: PandaGene,
    /// A mask of all states of the panda
    pub info: Mask<PandaInfo>,
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
