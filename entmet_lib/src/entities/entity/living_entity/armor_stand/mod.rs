use std::ops::{Deref, DerefMut};

use crate::datatypes::Rotations;

use super::LivingEntity;

#[repr(u8)]
pub enum ArmorStandInfo {
    Small = 0x01,
    Arms = 0x04,
    NoBasePlate = 0x08,
    Marker = 0x10,
}
pub struct ArmorStand {
    living_entity: LivingEntity,
    pub info: Vec<ArmorStandInfo>,
    pub head_rotation: Rotations,
    pub body_rotation: Rotations,
    pub left_arm_rotation: Rotations,
    pub right_arm_rotation: Rotations,
    pub left_leg_rotation: Rotations,
    pub right_leg_rotation: Rotations,
}
impl Deref for ArmorStand {
    type Target = LivingEntity;

    fn deref(&self) -> &Self::Target {
        &self.living_entity
    }
}
impl DerefMut for ArmorStand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity
    }
}
impl Default for ArmorStand {
    fn default() -> Self {
        Self {
            living_entity: LivingEntity::default(),
            info: Vec::new(),
            head_rotation: Rotations { x: 0.0, y: 0.0, z: 0.0 },
            body_rotation: Rotations { x: 0.0, y: 0.0, z: 0.0 },
            left_arm_rotation: Rotations { x: -10.0, y: 0.0, z: -10.0 },
            right_arm_rotation: Rotations { x: -15.0, y: 0.0, z: 10.0 },
            left_leg_rotation: Rotations { x: -1.0, y: 0.0, z: -1.0 },
            right_leg_rotation: Rotations { x: 1.0, y: 0.0, z: 1.0 },
        }
    }
}
