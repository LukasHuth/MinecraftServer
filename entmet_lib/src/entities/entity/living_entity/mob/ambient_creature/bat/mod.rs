use std::ops::{Deref, DerefMut};

use super::AmbientCreature;

pub struct Bat {
    ambient_crature: AmbientCreature,
    pub is_hanging: bool,
}
impl Deref for Bat {
    type Target = AmbientCreature;

    fn deref(&self) -> &Self::Target {
        &self.ambient_crature
    }
}
impl DerefMut for Bat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ambient_crature
    }
}
impl Default for Bat {
    fn default() -> Self {
        Self {
            ambient_crature: AmbientCreature::default(),
            is_hanging: false,
        }
    }
}
