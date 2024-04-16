use std::ops::{Deref, DerefMut};

use super::Animal;

#[derive(Default)]
pub struct Strider {
    animal: Animal,
    pub boost_time: i32,
    pub shaking: bool,
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
