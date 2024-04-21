use std::ops::{Deref, DerefMut};

use crate::datatypes::Mask;

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum FoxVariant {
    #[default] Red = 0,
    Snow = 1,
}
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

#[derive(Default)]
pub struct Fox {
    animal: Animal,
    pub variant: FoxVariant,
    pub info: Mask<FoxInfo>,
    pub first_uuid: Option<u128>,
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
