use std::ops::{Deref, DerefMut};

use super::Cow;


#[derive(Default)]
pub enum MooshroomVariants {
    #[default] Red,
    Brown,
}
#[derive(Default)]
pub struct Mooshroom {
    cow: Cow,
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
