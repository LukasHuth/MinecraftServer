use std::ops::{Deref, DerefMut};

use super::Animal;

/// The different states that a be can be in
#[repr(u8)]
#[allow(missing_docs)]
pub enum BeeState {
    IsAngry = 0x02,
    HasStung = 0x04,
    HasNectar = 0x08,
}

/// An instance of a Bee
#[derive(Default)]
pub struct Bee {
    animal: Animal,
    /// The state of the bee
    pub state: Option<BeeState>,
    /// Time in ticks, how long the bee will still be angry
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
