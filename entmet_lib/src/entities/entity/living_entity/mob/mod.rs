use super::LivingEntity;

pub mod ambient_creature;
pub mod pathfinder_mob;

#[repr(u8)]
pub enum MobInfo {
    NoAI = 0x01,
    LeftHanded = 0x02,
    Aggressive = 0x04,
}
pub struct Mob {
    living_entity: LivingEntity,
    info: Vec<MobInfo>,
}
impl Default for Mob {
    fn default() -> Self {
        Self {
            living_entity: LivingEntity::default(),
            info: Vec::new(),
        }
    }
}
