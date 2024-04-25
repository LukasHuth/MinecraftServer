use std::ops::{Deref, DerefMut};

use super::Animal;

/// An instance of a strider
#[derive(Default)]
pub struct Strider {
    animal: Animal,
    ///  how long the boost time lasts
    pub boost_time: i32,
    /// Whether the strider is shaking or not
    pub shaking: bool,
    /// Whether the strider has a saddle or not
    pub saddle: bool,
}
impl Deref for Strider {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Strider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
