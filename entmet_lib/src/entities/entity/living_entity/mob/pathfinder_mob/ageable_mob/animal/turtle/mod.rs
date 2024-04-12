use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Turtle {
    animal: Animal,
    pub home_pos: (i32, i32, i16),
    pub has_egg: bool,
    pub is_laying_egg: bool,
    pub travel_pos: (i32, i32, i16),
    pub is_going_home: bool,
    pub is_traveling: bool,
}
impl Deref for Turtle {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Turtle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
