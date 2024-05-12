//! Module for NBT serialization
//!
//! The following code is copied and modified from fastnbt:
//! <https://github.com/owengage/fastnbt/blob/da6d919ac3916d2a951ee79451497f4981802ca9/fastnbt/src/value/ser.rs>
//!
//! for which the license is MIT

use std::collections::HashMap;

use serde::{ser::Impossible, Serialize};

use crate::{error::Error, nbt_value::array_serializer::ArraySerializer, NbtTypeId, NbtValue};
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
            Self::Compound(_name, v) => {v.serialize(serializer)},
            Self::IntArray(v) => v.serialize(serializer),
            Self::LongArray(v) => v.serialize(serializer),
        }
    }
}

/// Struct for NBT Serialization
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

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
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
        _name: &'static str,
        _variant_index: u32,
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
    fn serialize_tuple_struct(self,_name: &'static str,len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str,len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            name: variant.into(),
            vec: Vec::with_capacity(len)
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: HashMap::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(self,_name: &'static str,len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str,_len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
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

/// Struct for Serializing a `Vec` into a [`NbtValue`]
///
/// [`NbtValue`]: crate::NbtValue
pub struct SerializeVec {
    vec: Vec<NbtValue>,
}

/// Struct for Serializing a `Tuple Variant` into a [`NbtValue`]
///
/// [`NbtValue`]: crate::NbtValue
pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<NbtValue>,
}

/// Struct for Serializing a `Map` into a [`NbtValue`]
///
/// [`NbtValue`]: crate::NbtValue
pub struct SerializeMap {
    map: HashMap<String, NbtValue>,
    next_key: Option<String>,
}

/// Struct for Serializing a `Struct Variant` into a [`NbtValue`]
///
/// [`NbtValue`]: crate::NbtValue
pub struct SerializeStructVariant {
    name: String,
    map: HashMap<String, NbtValue>,
}
impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.vec.push(crate::to_nbt_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtValue::List(self.vec))
    }
}
impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.vec.push(crate::to_nbt_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut object = HashMap::new();
        object.insert(self.name, NbtValue::List(self.vec));
        Ok(NbtValue::Compound(None, object))
    }
}
impl serde::ser::SerializeMap for SerializeMap {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.next_key = Some(key.serialize(MapKeySerializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        let key = self.next_key.take().expect("serialize_value called before serialize_key");
        self.map.insert(key, crate::to_nbt_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.map.len() != 1 {
            return Ok(NbtValue::Compound(None, self.map));
        }
        let (key, val) = self.map.iter().next().expect("This should never fail, because this code only executes with one element in the map");
        let data = || match val {
            NbtValue::List(bs) => bs.iter().map(|v| v.number_as_be_bytes().unwrap()).collect::<Vec<Vec<u8>>>(),
            _ => unreachable!(),
        };
        Ok(match key.as_str() {
            "__byte_array" => NbtValue::ByteArray(data().iter().flatten().map(|&e|e as i8).collect()),
            "__int_array" => NbtValue::IntArray(IntArrayHelper::from_bytes(data())?),
            "__long_array" => NbtValue::LongArray(LongArrayHelper::from_bytes(data())?),
            _ => NbtValue::Compound(None, self.map)
        })
    }
}
struct IntArrayHelper;
impl IntArrayHelper {
    fn from_bytes(values: Vec<Vec<u8>>) -> Result<Vec<i32>, Error> {
        let values: Vec<u8> = values.into_iter().flatten().collect();
        if values.len() & 3 != 0 { return Err(Error::Message("Invalid amount of bytes for a array list".to_string())); }
        Ok(values.chunks(4).map(|chunk|i32::from_be_bytes(chunk.try_into().expect("this should never fail, because of the previous check"))).collect())
    }
}
struct LongArrayHelper;
impl LongArrayHelper {
    fn from_bytes(values: Vec<Vec<u8>>) -> Result<Vec<i64>, Error> {
        let values: Vec<u8> = values.into_iter().flatten().collect();
        if values.len() & 7 != 0 { return Err(Error::Message("Invalid amount of bytes for a array list".to_string())); }
        Ok(values.chunks(8).map(|chunk|i64::from_be_bytes(chunk.try_into().expect("this should never fail, because of the previous check"))).collect())
    }
}
struct MapKeySerializer;
fn key_must_be_a_string() -> Error {
    Error::Message("Key must be a string".to_string())
}
impl serde::Serializer for MapKeySerializer {
    type Ok = String;

    type Error = Error;

    type SerializeSeq = Impossible<String, Error>;

    type SerializeTuple = Impossible<String, Error>;

    type SerializeTupleStruct = Impossible<String, Error>;

    type SerializeTupleVariant = Impossible<String, Error>;

    type SerializeMap = Impossible<String, Error>;

    type SerializeStruct = Impossible<String, Error>;

    type SerializeStructVariant = Impossible<String, Error>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_unit_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str,) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_owned())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self,_name: &'static str,value: &T,) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_tuple_struct(self,_name: &'static str,_len: usize,) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize,) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_struct(self,_name: &'static str,_len: usize,) -> Result<Self::SerializeStruct, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_struct_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize,) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + std::fmt::Display, {
        Ok(value.to_string())
    }
}

impl serde::ser::SerializeStruct for SerializeMap {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = NbtValue;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.map.insert(String::from(key), crate::to_nbt_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut object = HashMap::new();

        object.insert(self.name, NbtValue::Compound(None, self.map));

        Ok(NbtValue::Compound(None, object))
    }
}
