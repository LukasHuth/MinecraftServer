use std::ops::{Deref, DerefMut};

use super::Cow;

/// An enum of all Mooshroom variants
#[derive(Default)]
pub enum MooshroomVariants {
    /// The default red color
    #[default] Red,
    /// The rare brown color
    Brown,
}
/// An instance of a Mooshroom
#[derive(Default)]
pub struct Mooshroom {
    cow: Cow,
    /// The variant of the Mooshroom
    pub variant: MooshroomVariants,
}
impl Deref for Mooshroom {
    type Target = Cow;

    fn deref(&self) -> &Self::Target {
        &self.cow
    }
}
impl DerefMut for Mooshroom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cow
    }
}
