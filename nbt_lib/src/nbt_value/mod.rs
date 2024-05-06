//! This module contains all important functions for nbt values

use std::collections::HashMap;
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
    Compound(Option<String>, HashMap<String, NbtValue>),
    /// Data wrapper to store a list of signed 32-bit integers
    IntArray(Vec<i32>),
    /// Data wrapper to store a list of signed 64-bit integers
    LongArray(Vec<i64>),
}
pub mod ser;
///
pub mod de;
