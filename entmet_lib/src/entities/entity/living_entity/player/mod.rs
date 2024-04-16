use std::ops::{Deref, DerefMut};

use nbt_lib::NbtValue;

use super::LivingEntity;

#[derive(Clone, Copy)]
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


#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

pub struct Player {
    living_entity: LivingEntity,
    pub additional_hearts: f32,
    pub score: i32,
    pub displayed_skin_parts: Vec<DisplayedSkinParts>,
    pub main_hand: MainHand,
    pub left_shoulder: Option<NbtValue>,
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
            displayed_skin_parts: Vec::new(),
            main_hand: MainHand::Right,
            left_shoulder: None, // write 0
            right_shoulder: None,
        }
    }
}
