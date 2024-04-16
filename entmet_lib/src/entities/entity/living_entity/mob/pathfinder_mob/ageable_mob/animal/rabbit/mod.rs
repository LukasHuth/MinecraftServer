use std::ops::{Deref, DerefMut};

use super::Animal;

#[repr(u8)]
#[derive(Default)]
pub enum RabbitVariant {
    #[default] Brown = 0,
    White = 1,
    Black = 2,
    WhiteSplotched = 3,
    Gold = 4,
    Salt = 5,
    Evil = 99,
}

#[derive(Default)]
pub struct Rabbit {
    animal: Animal,
    pub variant: RabbitVariant,
}
impl Deref for Rabbit {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
impl DerefMut for Rabbit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal
    }
}
