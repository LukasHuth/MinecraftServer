use std::ops::{Deref, DerefMut};

use super::Animal;

/// An enum of all rabbit variants
#[allow(missing_docs)]
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

/// An instance of a rabbit
#[derive(Default)]
pub struct Rabbit {
    animal: Animal,
    /// The variant of the rabbit
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
