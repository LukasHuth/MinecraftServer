//! Module for NBT array serialization
//!
//! The following code is copied and modified from fastnbt:
//! <https://github.com/owengage/fastnbt/blob/da6d919ac3916d2a951ee79451497f4981802ca9/fastnbt/src/value/array_serializer.rs>
//!
//! for which the license is MIT
use byteorder::{BigEndian, ReadBytesExt};
use serde::ser::Impossible;

use crate::{NbtTypeId, NbtValue, error::Error};

/// A struct for serializing an array into a [`NbtValue`]
///
/// [`NbtValue`]: `crate::NbtValue`
pub struct ArraySerializer<'a> {
    /// The serializer instance
    pub ser: &'a mut crate::nbt_value::ser::Serializer,
    /// The type of the array that is getting serialized
    pub tag: NbtTypeId,
}

impl<'a> serde::Serializer for ArraySerializer<'a> {
    type Ok = NbtValue;

    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    type SerializeTuple = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Impossible<Self::Ok, Self::Error>;

    type SerializeStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.tag {
            NbtTypeId::ByteArray => Ok(NbtValue::ByteArray(v.iter().map(|&e| e as i8).collect())),
            NbtTypeId::IntArray => Ok(NbtValue::IntArray(v.chunks_exact(4).map(|mut bs| bs.read_i32::<BigEndian>()).collect::<std::io::Result<Vec<i32>>>()?)),
            NbtTypeId::LongArray => Ok(NbtValue::LongArray(v.chunks_exact(8).map(|mut bs| bs.read_i64::<BigEndian>()).collect::<std::io::Result<Vec<i64>>>()?)),
            _ => unreachable!(),
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self,_name: &'static str,_value: &T,) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T>(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_value: &T,) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self,_name: &'static str,_len: usize,) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize,) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self,_name: &'static str,_len: usize,) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize,) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
