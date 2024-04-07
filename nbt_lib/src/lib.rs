pub mod reader;
pub mod writer;
pub mod error;
pub mod version;
pub mod datatypes;

#[cfg(test)]
mod tests;

pub mod traits;

pub type NbtTypeId = u8;

#[derive(Debug, Clone)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(Option<String>, Vec<(String, NbtValue)>),
    IntArray(Vec<i32>),
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
