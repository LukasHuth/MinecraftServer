use std::ops::{Deref, DerefMut};

use super::Animal;

/// An instance of a turtle
#[derive(Default)]
pub struct Turtle {
    animal: Animal,
    /// The position of the home of the turtle
    pub home_pos: (i32, i32, i16),
    /// Whether the turtle has eggs or not
    pub has_egg: bool,
    /// Whether the turtle is currently laying eggs or not
    pub is_laying_egg: bool,
    /// The position that the turle is aming to travel to
    pub travel_pos: (i32, i32, i16),
    /// Whether the turtle is going home or not
    pub is_going_home: bool,
    /// Whether the turtle is traveling or not
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
