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
    info: Vec<ArmorStandInfo>,
    head_rotation: Rotations,
    body_rotation: Rotations,
    left_arm_rotation: Rotations,
    right_arm_rotation: Rotations,
    left_leg_rotation: Rotations,
    right_leg_rotation: Rotations,
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
