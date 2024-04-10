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
    additional_hearts: f32,
    score: i32,
    displayed_skin_parts: Vec<DisplayedSkinParts>,
    main_hand: MainHand,
    left_shoulder: NbtValue,
    right_shoulder: NbtValue,
}
impl Default for Player {
    fn default() -> Self {
        todo!()
    }
}
