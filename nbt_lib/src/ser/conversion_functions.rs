use serde::Serialize;
use std::fmt::Debug;
use crate::error::Result;

use super::Serializer;

/// Serialize some `T` into NBT data.
#[inline]
pub fn to_bytes<T: Serialize + Debug>(v: &T) -> Result<Vec<u8>> {
    to_bytes_with_opts(v, Default::default())
}

/// Options for customizing serialization.
#[derive(Default, Debug, Clone)]
pub struct SerOpts {
    root_name: String,
}

impl SerOpts {
    /// Create new options. This object follows a builder pattern.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the root name (top level) of the compound. In most Minecraft data
    /// structures this is the empty string. The [`ser`][`crate::ser`] module
    /// contains an example.
    pub fn root_name(mut self, root_name: impl Into<String>) -> Self {
        self.root_name = root_name.into();
        self
    }
}
impl From<&str> for SerOpts {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
impl From<String> for SerOpts {
    fn from(root_name: String) -> Self {
        Self { root_name }
    }
}

/// converts data implementing [`Serialize`] into NBT data bytes
///
/// [`Serialize`]: `serde::Serialize`
pub fn to_bytes_with_opts<T: Serialize + Debug>(v: &T, opts: SerOpts) -> Result<Vec<u8>> {
    let mut result = vec![];
    let mut serializer = Serializer {
        writer: &mut result,
        root_name: opts.root_name
    };
    v.serialize(&mut serializer)?;
    Ok(result)
}
