use std::ops::{Deref, DerefMut};

use super::Guardian;

/// An instance of an elder guardian
#[derive(Default)]
pub struct ElderGuardian {
    guardian: Guardian,
}
impl Deref for ElderGuardian {
    type Target = Guardian;

    fn deref(&self) -> &Self::Target {
        &self.guardian
    }
}
impl DerefMut for ElderGuardian {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guardian
    }
}
