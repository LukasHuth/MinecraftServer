use std::ops::{Deref, DerefMut};

use super::Monster;

/// An enum of all creeper states
#[allow(missing_docs)]
#[repr(i8)]
#[derive(Default)]
pub enum CreeperState {
    #[default] Idle = -1,
    Fuse = 1,
}

/// An instance of a creeper
#[derive(Default)]
pub struct Creeper {
    monster: Monster,
    /// The state of the creeper
    pub state: CreeperState,
    /// Whether it is charging or not
    pub charged: bool,
    /// Whether it is ignited or not
    pub ignited: bool,
}
impl Deref for Creeper {
    type Target = Monster;

    fn deref(&self) -> &Self::Target {
        &self.monster
    }
}
impl DerefMut for Creeper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster
    }
}
