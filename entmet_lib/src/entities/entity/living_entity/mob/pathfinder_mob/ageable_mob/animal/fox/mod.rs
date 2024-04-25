use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Animal;

/// An enum of all fox variants
#[repr(u8)]
#[derive(Default)]
pub enum FoxVariant {
    /// The normal variant
    #[default] Red = 0,
    /// Snow fox variant
    Snow = 1,
}
/// An enum of all states the fox can be in
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FoxInfo {
    IsSitting = 0x01,
    IsCrouching = 0x04,
    IsInterested = 0x08,
    IsPouncing = 0x10,
    IsSleeping = 0x20,
    IsFaceplanted = 0x40,
    IsDefending = 0x80,
}
impl Into<u8> for FoxInfo {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An instance of a Fox
#[derive(Default)]
pub struct Fox {
    animal: Animal,
    /// The variant of the fox
    pub variant: FoxVariant,
    /// A mask of all informations about the fox
    pub info: Mask<FoxInfo>,
    /// uuid information 1
    pub first_uuid: Option<u128>,
    /// uuid information 2
    pub second_uuid: Option<u128>,
}
impl Deref for Fox {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Fox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
