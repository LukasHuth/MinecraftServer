use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
pub enum BeeState {
    IsAngry = 0x02,
    HasStung = 0x04,
    HasNectar = 0x08,
}

#[derive(Default)]
pub struct Bee {
    animal: Animal,
    pub state: Option<BeeState>,
    pub anger_time: i32,
}
impl Deref for Bee {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Bee {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
