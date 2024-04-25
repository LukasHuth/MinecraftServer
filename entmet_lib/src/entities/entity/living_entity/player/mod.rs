use std::ops::{Deref, DerefMut};

use nbt_lib::NbtValue;

use crate::datatypes::Mask;

use super::LivingEntity;

/// An enum of the displayable skin parts
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DisplayedSkinParts {
    Cape = 0x01,
    Jacket = 0x02,
    LeftSleve = 0x04,
    RightSleve = 0x08,
    LeftPantsLeg = 0x10,
    RightPantsLeg = 0x20,
    Hat = 0x40,
    Unused = 0x80,
}
impl Into<u8> for DisplayedSkinParts {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An enum of the possible main hands
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

/// An instance of a player
pub struct Player {
    living_entity: LivingEntity,
    /// How many additional hearts the player has
    pub additional_hearts: f32,
    /// The score of the player
    pub score: i32,
    /// A list of all displayed skin parts
    pub displayed_skin_parts: Mask<DisplayedSkinParts>,
    /// The main hand of the player
    pub main_hand: MainHand,
    /// NbtData of optional things on the left shoulder
    pub left_shoulder: Option<NbtValue>,
    /// NbtData of optional things on the right shoulder
    pub right_shoulder: Option<NbtValue>,
}
impl Deref for Player {
    type Target = LivingEntity;

    fn deref(&self) -> &Self::Target {
        &self.living_entity
    }
}
impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            living_entity: LivingEntity::default(),
            additional_hearts: 0.0,
            score: 0,
            displayed_skin_parts: Mask::default(),
            main_hand: MainHand::Right,
            left_shoulder: None, // write 0
            right_shoulder: None,
        }
    }
}
