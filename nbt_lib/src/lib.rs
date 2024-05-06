#![deny(missing_docs)]
//! This crate allows reading and writing minecraft NBT (Named Binary Tag) data

use std::fmt::LowerHex;

/// Module for Deserialization
pub mod de;
/// Module for Serialization
pub mod ser;

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
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NbtTypeId {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl std::fmt::Display for NbtTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", *self as u8))
    }
}
impl LowerHex for NbtTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x}", *self as u8))
    }
}

/// The id of the current nbt version
///
/// # Info
///
/// this number is according to [this article](https://minecraft.fandom.com/wiki/NBT_format#History)
pub const NBT_VERSION: i32 = 19133;

pub mod nbt_value;
pub use nbt_value::NbtValue;

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
