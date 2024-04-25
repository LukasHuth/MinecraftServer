#![deny(missing_docs)]
//! This crate allows reading and writing minecraft NBT (Named Binary Tag) data

/// Module that implements the functions to read NBT data
pub mod reader;

/// Module that implements the functions to write NBT data
pub mod writer;

/// Module containing all structs neccessary for error handling
pub mod error;

/// Module defining the different version that can be used the read and write NBT
pub mod version;

/// Module defining NBT specific datatypes
pub mod datatypes;

#[cfg(test)]
mod tests;

/// Module containing all trait declarations
pub mod traits;

/// type cast to give the NbtValue type id and undestandable name
pub type NbtTypeId = u8;

/// The id of the current nbt version
///
/// # Info
///
/// this number is according to [this article](https://minecraft.fandom.com/wiki/NBT_format#History)
pub const NBT_VERSION: i32 = 19133;

/// Enum storing NBT data
#[derive(Debug, Clone)]
pub enum NbtValue {
    /// Data wrapper to store a signed 8-bit integer
    Byte(i8),
    /// Data wrapper to store a signed 16-bit integer
    Short(i16),
    /// Data wrapper to store a signed 32-bit integer
    Int(i32),
    /// Data wrapper to store a signed 64-bit integer
    Long(i64),
    /// Data wrapper to store a signed 32-bit floating-point value
    Float(f32),
    /// Data wrapper to store a signed 64-bit floating-point value
    Double(f64),
    /// Data wrapper to store a list of signed 8-bit integers
    ByteArray(Vec<i8>),
    /// Data wrapper to store a string
    String(String),
    /// Data wrapper to store a list of NBT values
    List(Vec<NbtValue>),
    /// Data wrapper to store a named list of NBT values
    ///
    /// # Note
    ///
    /// This struct can also have a name
    Compound(Option<String>, Vec<(String, NbtValue)>),
    /// Data wrapper to store a list of signed 32-bit integers
    IntArray(Vec<i32>),
    /// Data wrapper to store a list of signed 64-bit integers
    LongArray(Vec<i64>),
}
macro_rules! assert_return_IEEE754 {
    ($v0:expr, $v1:expr) => {
        if $v0 != $v1 {
            if $v0.is_nan() && $v1.is_nan() { return true }
            eprintln!("Value {:?} and {:?} are not equal", $v0, $v1);
            false
        } else { true }
    };
}
macro_rules! assert_return {
    ($v0:expr, $v1:expr) => {
        if $v0 != $v1 {
            eprintln!("Value {:?} and {:?} are not equal", $v0, $v1);
            false
        } else { true }
    };
}
impl PartialEq for NbtValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Byte(v0), Self::Byte(v1)) => assert_return!(v0, v1),
            (Self::Short(v0), Self::Short(v1)) => assert_return!(v0, v1),
            (Self::Int(v0), Self::Int(v1)) => assert_return!(v0, v1),
            (Self::Long(v0), Self::Long(v1)) => assert_return!(v0, v1),
            (Self::Float(v0), Self::Float(v1)) => assert_return_IEEE754!(v0, v1),
            (Self::Double(v0), Self::Double(v1)) => assert_return_IEEE754!(v0, v1),
            (Self::String(v0), Self::String(v1)) => assert_return!(v0, v1),
            (Self::ByteArray(v0), Self::ByteArray(v1)) => assert_return!(v0, v1),
            (Self::IntArray(v0), Self::IntArray(v1)) => assert_return!(v0, v1),
            (Self::LongArray(v0), Self::LongArray(v1)) => assert_return!(v0, v1),
            (Self::List(v0), Self::List(v1)) => assert_return!(v0, v1),
            (Self::Compound(name0, v0), Self::Compound(name1, v1)) => {
                if v0.len() != v1.len() { return false }
                if name0 != name1 { return false }
                for value in v0 {
                    if !v1.contains(value) { return false }
                }
                true
            }
            _ => false
        }
    }
}
mod nbt_implementation;
