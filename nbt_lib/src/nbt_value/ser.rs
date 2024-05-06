//! Module for NBT serialization

use std::collections::HashMap;

use serde::Serialize;

use crate::{error::Error, NbtTypeId, NbtValue};
impl Serialize for NbtValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Self::Byte(v) => serializer.serialize_i8(*v),
            Self::Short(v) => serializer.serialize_i16(*v),
            Self::Int(v) => serializer.serialize_i32(*v),
            Self::Long(v) => serializer.serialize_i64(*v),
            Self::Float(v) => serializer.serialize_f32(*v),
            Self::Double(v) => serializer.serialize_f64(*v),
            Self::String(v) => serializer.serialize_str(v),
            Self::ByteArray(v) => v.serialize(serializer),
            Self::List(v) => v.serialize(serializer),
            // Info: name is only set on the root compound, it's stripped here for simplicity
            Self::Compound(_name, v) => v.serialize(serializer),
            Self::IntArray(v) => v.serialize(serializer),
            Self::LongArray(v) => v.serialize(serializer),
        }
    }
}
pub struct Serializer;

impl<'a> serde::Serializer for &'a mut Serializer {
    type Ok = NbtValue;

    type Error = Error;

    type SerializeSeq = SerializeVec;

    type SerializeTuple = SerializeVec;

    type SerializeTupleStruct = SerializeVec;

    type SerializeTupleVariant = SerializeTupleVariant;

    type SerializeMap = SerializeMap;

    type SerializeStruct = SerializeMap;

    type SerializeStructVariant = SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Byte(v as i8))
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Byte(v))
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Short(v))
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Int(v))
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Long(v))
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.serialize_u128(v as u128)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::IntArray(vec![
            (v >> 96) as i32,
            (v >> 64) as i32,
            (v >> 32) as i32,
            v as i32,
        ]))
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Byte(v as i8))
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Short(v as i16))
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Int(v as i32))
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Long(v as i64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::Int(v as i32))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::String(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::List(
            v.iter().map(|b|NbtValue::Byte(*b as i8)).collect()
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[inline]
    fn serialize_unit_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self,_name: &'static str,value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        match variant {
            "__byte_array" => value.serialize(ArraySerializer {
                ser: self,
                tag: NbtTypeId::ByteArray
            }),
            "__int_array" => value.serialize(ArraySerializer {
                ser: self,
                tag: NbtTypeId::IntArray
            }),
            "__long_array" => value.serialize(ArraySerializer {
                ser: self,
                tag: NbtTypeId::LongArray
            }),
            _ => todo!()
        }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0))
        })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(self,name: &'static str,len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self,name: &'static str,variant_index: u32,variant: &'static str,len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            name: variant.into(),
            vec: Vec::with_capacity(len)
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: HashMap::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(self,name: &'static str,len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(self,name: &'static str,variant_index: u32,variant: &'static str,len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant {
            name: variant.into(),
            map: HashMap::new(),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + std::fmt::Display, {
        Ok(NbtValue::String(value.to_string()))
    }
}
