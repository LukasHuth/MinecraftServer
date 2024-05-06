use std::collections::HashMap;

use crate::{NbtValue, error::{NbtResult, NbtError}, traits::{NbtRead, NbtWrite}, reader::NbtReader, NbtTypeId};

impl NbtValue {
    /// function to convery a list of bytes into a `NbtResult` containing the read `NbtValue`
    pub fn from_binary<R>(data: Vec<u8>) -> NbtResult<NbtValue> where R: NbtRead {
        let reader = NbtReader::new(data);
        R::from_reader(reader)
    }
    /// function to get the tag of the NbtValue
    pub fn tag(&self) -> NbtTypeId {
        match self {
            NbtValue::Byte(_) => NbtTypeId::Byte,
            NbtValue::Short(_) => NbtTypeId::Short,
            NbtValue::Int(_) => NbtTypeId::Int,
            NbtValue::Long(_) => NbtTypeId::Long,
            NbtValue::Float(_) => NbtTypeId::Float,
            NbtValue::Double(_) => NbtTypeId::Double,
            NbtValue::ByteArray(_) => NbtTypeId::ByteArray,
            NbtValue::String(_) => NbtTypeId::String,
            NbtValue::List(_) => NbtTypeId::List,
            NbtValue::Compound(_, _) => NbtTypeId::Compound,
            NbtValue::IntArray(_) => NbtTypeId::IntArray,
            NbtValue::LongArray(_) => NbtTypeId::LongArray,
        }
    }
    /// function to write nbt data to a type that implements `NbtWrite`
    pub fn write_to<W>(&self, buff: &mut Vec<u8>) -> NbtResult<()> where W: NbtWrite {
        W::write_to(self, buff)
    }
    /// function to write nbt data with an name to a type that implements `NbtWrite`
    pub fn write_to_with_name<W>(&self, name: &str, buff: &mut Vec<u8>) -> NbtResult<()> where W: NbtWrite {
        W::write_to_with_name(name, self, buff)
    }
    /// function to convert nbt data to a `NbtResult` containing a list of bytes, representing the
    /// nbt data
    pub fn to_binary<W>(&self) -> NbtResult<Vec<u8>> where W: NbtWrite {
        W::to_bytes(self)
    }
    /// function to get the `NbtValue` as an i8
    #[inline]
    pub fn as_i8(&self) -> NbtResult<i8> {
        match self {
            NbtValue::Byte(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Byte, self.tag())),
        }
    }
    /// function to get the `NbtValue` as an i16
    #[inline]
    pub fn as_i16(&self) -> NbtResult<i16> {
        match self {
            NbtValue::Short(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Short, self.tag())),
        }
    }
    /// function to get the `NbtValue` as an i32
    #[inline]
    pub fn as_i32(&self) -> NbtResult<i32> {
        match self {
            NbtValue::Int(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Int, self.tag())),
        }
    }
    /// function to get the `NbtValue` as an i64
    #[inline]
    pub fn as_i64(&self) -> NbtResult<i64> {
        match self {
            NbtValue::Long(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Long, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a f32
    #[inline]
    pub fn as_f32(&self) -> NbtResult<f32> {
        match self {
            NbtValue::Float(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Float, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a f64
    #[inline]
    pub fn as_f64(&self) -> NbtResult<f64> {
        match self {
            NbtValue::Double(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Double, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a list of i8's
    #[inline]
    pub fn as_i8_array(&self) -> NbtResult<Vec<i8>> {
        match self {
            NbtValue::ByteArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(NbtTypeId::ByteArray, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a list of i32's
    #[inline]
    pub fn as_i32_array(&self) -> NbtResult<Vec<i32>> {
        match self {
            NbtValue::IntArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(NbtTypeId::IntArray, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a list of i64's
    #[inline]
    pub fn as_i64_array(&self) -> NbtResult<Vec<i64>> {
        match self {
            NbtValue::LongArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(NbtTypeId::LongArray, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a string
    #[inline]
    pub fn as_string(&self) -> NbtResult<String> {
        match self {
            NbtValue::String(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(NbtTypeId::String, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a list of `NbtValue`'s
    #[inline]
    pub fn as_list(&self) -> NbtResult<Vec<NbtValue>> {
        match self {
            NbtValue::List(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(NbtTypeId::List, self.tag())),
        }
    }
    /// function to get the `NbtValue` as a list of named `NbtValue`'s
    #[inline]
    pub fn as_compound(&self) -> NbtResult<(Option<&String>, HashMap<String, NbtValue>)> {
        match self {
            NbtValue::Compound(name, v) => Ok((name.as_ref(), v.clone())),
            _ => Err(NbtError::IncorrectType(NbtTypeId::Compound, self.tag())),
        }
    }
    /// function to determine, if the `NbtValue` is a `i8`
		#[inline]
    pub fn is_i8(&self) -> bool { matches!(self, NbtValue::Byte(_)) }
    /// function to determine, if the `NbtValue` is a `i16`
		#[inline]
    pub fn is_i16(&self) -> bool { matches!(self, NbtValue::Short(_)) }
    /// function to determine, if the `NbtValue` is a `i32`
		#[inline]
    pub fn is_i32(&self) -> bool { matches!(self, NbtValue::Int(_)) }
    /// function to determine, if the `NbtValue` is a `i64`
		#[inline]
    pub fn is_i64(&self) -> bool { matches!(self, NbtValue::Long(_)) }
    /// function to determine, if the `NbtValue` is a `f32`
		#[inline]
    pub fn is_f32(&self) -> bool { matches!(self, NbtValue::Float(_)) }
    /// function to determine, if the `NbtValue` is a `f64`
		#[inline]
    pub fn is_f64(&self) -> bool { matches!(self, NbtValue::Double(_)) }
    /// function to determine, if the `NbtValue` is a `i8 array`
		#[inline]
    pub fn is_i8_array(&self) -> bool { matches!(self, NbtValue::ByteArray(_)) }
    /// function to determine, if the `NbtValue` is a `i32 array`
		#[inline]
    pub fn is_i32_array(&self) -> bool { matches!(self, NbtValue::IntArray(_)) }
    /// function to determine, if the `NbtValue` is a `i64 array`
		#[inline]
    pub fn is_i64_array(&self) -> bool { matches!(self, NbtValue::LongArray(_)) }
    /// function to determine, if the `NbtValue` is a `String`
		#[inline]
    pub fn is_string(&self) -> bool { matches!(self, NbtValue::String(_)) }
    /// function to determine, if the `NbtValue` is a `List`
		#[inline]
    pub fn is_list(&self) -> bool { matches!(self, NbtValue::List(_)) }
    /// function to determine, if the `NbtValue` is a `Compound`
		#[inline]
    pub fn is_compound(&self) -> bool { matches!(self, NbtValue::Compound(_, _)) }
}
