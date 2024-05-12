#![deny(missing_docs)]
//! This crate allows reading and writing minecraft NBT (Named Binary Tag) data

use std::fmt::LowerHex;

pub mod de;
pub mod ser;

pub mod reader;

pub mod writer;

pub mod error;

pub mod version;

pub mod datatypes;

mod nbt_implementation;

#[cfg(test)]
mod tests;

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
impl TryFrom<u8> for NbtTypeId {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NbtTypeId::End),
            1 => Ok(NbtTypeId::Byte),
            2 => Ok(NbtTypeId::Short),
            3 => Ok(NbtTypeId::Int),
            4 => Ok(NbtTypeId::Long),
            5 => Ok(NbtTypeId::Float),
            6 => Ok(NbtTypeId::Double),
            7 => Ok(NbtTypeId::ByteArray),
            8 => Ok(NbtTypeId::String),
            9 => Ok(NbtTypeId::List),
            10 => Ok(NbtTypeId::Compound),
            11 => Ok(NbtTypeId::IntArray),
            12 => Ok(NbtTypeId::LongArray),
            13.. => Err(error::Error::Message(format!("Failed to convert tag id('{value}') into tag")))
        }
    }
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
pub use nbt_value::to_nbt_value;

macro_rules! assert_return_IEEE754 {
    ($v0:expr, $v1:expr) => {
        if $v0 != $v1 {
            if $v0.is_nan() && $v1.is_nan() { return true }
            println!("Value {:?} and {:?} are not equal", $v0, $v1);
            dbg!(false)
        } else { true }
    };
}
macro_rules! assert_return {
    ($v0:expr, $v1:expr) => {
        if $v0 != $v1 {
            println!("Value {:?} and {:?} are not equal", $v0, $v1);
            dbg!(false)
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
            (Self::Compound(_name0, v0), Self::Compound(_name1, v1)) => {
                if v0.len() != v1.len() { return dbg!(false) }
                for value in v0 {
                    if !v1.contains_key(value.0) { return dbg!(false) }
                    if value.1 != v1.get(value.0).unwrap() { return dbg!(false) }
                }
                true
            }
            _ => dbg!(false)
        }
    }
}
