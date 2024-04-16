use std::ops::{Deref, DerefMut};

use super::Monster;

#[repr(i8)]
#[derive(Default)]
pub enum CreeperState {
    #[default] Idle = -1,
    Fuse = 1,
}

#[derive(Default)]
pub struct Creeper {
    monster: Monster,
    pub state: CreeperState,
    pub charged: bool,
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
