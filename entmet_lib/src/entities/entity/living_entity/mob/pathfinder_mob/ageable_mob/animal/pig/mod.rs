use std::ops::{Deref, DerefMut};

use super::Animal;

/// An instance of a pig
#[derive(Default)]
pub struct Pig {
    animal: Animal,
    /// whether it has a saddle or not
    pub saddle: bool,
    /// the time until the boost runs out
    pub total_boost_time: i32,
}
impl Deref for Pig {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Pig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
