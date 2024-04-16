use std::ops::{Deref, DerefMut};

use super::Mob;

/// Ender Dragon Phase
/// for more info refer to this:
/// https://minecraft.wiki/w/Ender_Dragon#Data_values
/// and this:
/// https://wiki.vg/Entity_metadata#Ender_Dragon
///
#[repr(u8)]
#[derive(Default)]
pub enum EnderDragonPhase {
    Circling = 0,
    Strafing = 1,
    FlyingToThePortalToLand = 2,
    LandingOnThePortal = 3,
    TakingOffFromThePortal = 4,
    LandedPerformingBreathAttack = 5,
    LandedLookingForAPlayerForBreathAttack = 6,
    LandedRoarBeforeBeginningBreathAttack = 7,
    ChargingPlayer = 8,
    FlyingToPortalToDie = 9,
    #[default] HoveringWithNoAI = 10,
}

#[derive(Default)]
pub struct EnderDragon {
    mob: Mob,
    pub phase: EnderDragonPhase,
}
impl Deref for EnderDragon {
    type Target = Mob;

    fn deref(&self) -> &Self::Target {
        &self.mob
    }
}
impl DerefMut for EnderDragon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob
    }
}
