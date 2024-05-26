use std::ops::{Deref, DerefMut};

use crate::datatypes::{Mask, Rotations};

use super::LivingEntity;

/// An enum of the options of an armor stand
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArmorStandInfo {
    /// Whether it is small or not
    Small = 0x01,
    /// Whether it has arms or not
    Arms = 0x04,
    /// Whether it has a baseplate or not
    NoBasePlate = 0x08,
    /// Whether it is a marker or not
    Marker = 0x10,
}
impl Into<u8> for ArmorStandInfo {
    fn into(self) -> u8 {
        self as u8
    }
}

/// An instance of an armor stand
#[derive(PartialEq)]
pub struct ArmorStand {
    living_entity: LivingEntity,
    /// An vector of the armor stand options
    pub info: Mask<ArmorStandInfo>,
    /// The rotation of the head
    pub head_rotation: Rotations,
    /// The rotation of the head
    pub body_rotation: Rotations,
    /// The rotation of the left arm
    pub left_arm_rotation: Rotations,
    /// The rotation of the right arm
    pub right_arm_rotation: Rotations,
    /// The rotation of the left leg
    pub left_leg_rotation: Rotations,
    /// The rotation of the right leg
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
            info: Mask::default(),
            head_rotation: Rotations { x: 0.0, y: 0.0, z: 0.0 },
            body_rotation: Rotations { x: 0.0, y: 0.0, z: 0.0 },
            left_arm_rotation: Rotations { x: -10.0, y: 0.0, z: -10.0 },
            right_arm_rotation: Rotations { x: -15.0, y: 0.0, z: 10.0 },
            left_leg_rotation: Rotations { x: -1.0, y: 0.0, z: -1.0 },
            right_leg_rotation: Rotations { x: 1.0, y: 0.0, z: 1.0 },
        }
    }
}
