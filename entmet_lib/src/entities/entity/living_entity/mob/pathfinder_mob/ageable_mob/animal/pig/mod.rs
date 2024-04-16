use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Pig {
    animal: Animal,
    pub saddle: bool,
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
