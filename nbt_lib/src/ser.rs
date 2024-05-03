use std::io::Write;

use serde::{ser, Serialize, Serializer };
use serde::ser::{Impossible, SerializeSeq, SerializeStruct};

use byteorder::{BigEndian, WriteBytesExt};

use crate::error::{Error, Result};
use crate::version::Java;
use crate::{NbtTypeId, NbtValue};
mod constants;
pub use constants::*;

mod write_nbt_trait;
use write_nbt_trait::WriteNbt;

mod name_serializer;

/// 
#[warn(missing_docs)]
pub struct Serializer<W: std::io::Write> {
    pub(crate) writer: W,

    // Desired name of the root compound, typically an empty string.
    // NOTE: This is `mem:take`en, so is only valid at the start of serialization!
    pub(crate) root_name: String,
}
#[warn(missing_docs)]
pub struct SerializerTuple<'a, W: std::io::Write> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) len: usize,
    pub(crate) first: bool,
}
#[warn(missing_docs)]
pub struct SerializerMap<'a, W: std::io::Write> {
    ser: &'a mut Serializer<W>,
    key: Option<Vec<u8>>,
    header: Option<DelayedHeader>,
    trailer: Option<NbtTypeId>,
}
#[warn(missing_docs)]
struct Delayed<'a, W: std::io::Write + 'a> {
    ser: &'a mut Serializer<W>,
    header: Option<DelayedHeader>,
    is_list: bool,
}
#[warn(missing_docs)]
enum DelayedHeader {
    List { len: usize },
    MapEntry { outer_name: Vec<u8> },
    Root { root_name: String },
}

macro_rules! generate_no_root_function {
    ($name:ident($($type:ty),* $(,)?) -> $return_type:ty) => {
        paste::paste!{
            fn [< serialize_ $name >](self, $(_: $type),*) -> Result<$return_type> {
                Err(Self::Error::no_root_compound())
            }
        }
    };
    ($name:ident<T>($($type:ty),* $(,)?)) => {
        paste::paste!{
            fn [< serialize_ $name >]<T: ?Sized>(self, $(_: $type,)*) -> Result<Self::Ok> {
                Err(Self::Error::no_root_compound())
            }
        }
    };
    ($name:ident($($type:ty),* $(,)?)) => {
        generate_no_root_function!{$name($($type,)*) -> Self::Ok}
    };
}

impl<'a, W: 'a + std::io::Write> serde::ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();

    type Error = crate::error::Error;

    type SerializeSeq = SerializerTuple<'a, W>;

    type SerializeTuple = SerializerTuple<'a, W>;

    type SerializeTupleStruct = SerializerTuple<'a, W>;

    type SerializeTupleVariant = SerializerTuple<'a, W>;

    type SerializeMap = SerializerMap<'a, W>;

    type SerializeStruct = SerializerMap<'a, W>;

    type SerializeStructVariant = Impossible<(), crate::error::Error>;

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let root_name = self.root_name.clone();
        Ok(SerializerMap {
            ser: self,
            key: None,
            header: Some(DelayedHeader::Root { root_name }),
            trailer: Some(NbtTypeId::End),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    generate_no_root_function!(bool(bool));
    generate_no_root_function!(i8(i8));
    generate_no_root_function!(i16(i16));
    generate_no_root_function!(i32(i32));
    generate_no_root_function!(i64(i64));
    generate_no_root_function!(i128(i128));
    generate_no_root_function!(u8(u8));
    generate_no_root_function!(u16(u16));
    generate_no_root_function!(u32(u32));
    generate_no_root_function!(u64(u64));
    generate_no_root_function!(u128(u128));
    generate_no_root_function!(f32(f32));
    generate_no_root_function!(f64(f64));
    generate_no_root_function!(char(char));
    generate_no_root_function!(str(&str));
    generate_no_root_function!(bytes(&[u8]));
    generate_no_root_function!(none());
    generate_no_root_function!(some<T>(&T));
    generate_no_root_function!(unit());
    generate_no_root_function!(unit_struct(&'static str));
    generate_no_root_function!(unit_variant(&'static str, u32, &'static str));
    generate_no_root_function!(newtype_variant<T>(&'static str, u32, &'static str, &T));
    generate_no_root_function!(seq(Option<usize>) -> Self::SerializeSeq);
    generate_no_root_function!(tuple(usize) -> Self::SerializeSeq);
    generate_no_root_function!(tuple_struct(&'static str, usize) -> Self::SerializeTupleStruct);
    generate_no_root_function!(tuple_variant(&'static str, u32, &'static str, usize) -> Self::SerializeTupleStruct);
    generate_no_root_function!(struct_variant(&'static str, u32, &'static str, usize) -> Self::SerializeStructVariant);
}
fn write_header(writer: &mut impl std::io::Write, header: DelayedHeader, actual_tag: NbtTypeId) -> Result<()> {
    match header {
        DelayedHeader::Root { root_name } => {
            if actual_tag != NbtTypeId::Compound {
                return Err(crate::error::Error::no_root_compound());
            }
            writer.write_tag(NbtTypeId::Compound)?;
            writer.write_nbt_string(&root_name)?;
        }
        DelayedHeader::MapEntry { ref outer_name } => {
            writer.write_tag(actual_tag)?;
            writer.write_u16_be(outer_name.len() as u16)?;
            writer.write_bytes(&outer_name)?;
        }
        DelayedHeader::List { len } => {
            writer.write_tag(actual_tag)?;
            writer.write_len(len)?;
        }
    }
    Ok(())
}
impl<'a, W: Write> serde::ser::SerializeMap for SerializerMap<'a, W> {
    type Ok = ();

    type Error = crate::error::Error;

    fn serialize_key<T>(&mut self, key: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        let mut name = Vec::new();
        key.serialize(&mut name_serializer::NameSerializer { name: &mut name })?;
        self.key = Some(name);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        let name = self.key.take().ok_or_else(|| {
            crate::error::Error::Message("failed to get key".to_string())
        })?;
        dbg!(&name);
        let outer_tag = match std::str::from_utf8(&name) {
            _ => NbtTypeId::Compound,
        };
        if let Some(header) = self.header.take() {
            write_header(&mut self.ser.writer, header, outer_tag)?;
        }
        match std::str::from_utf8(&name) {
            /* byte_array */
            /* int array */
            /* long array */
            _ => value.serialize(&mut Delayed {
                ser: &mut *self.ser,
                header: Some(DelayedHeader::MapEntry { outer_name: name }),
                is_list: false,
            }),
        }
    }

    fn end(mut self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        if let Some(tag) = self.trailer {
            if let Some(header) = self.header.take() {
                write_header(&mut self.ser.writer, header, NbtTypeId::Compound)?;
            }
            self.ser.writer.write_tag(tag)?;
        }
        Ok(())
    }
}
impl<'a, W: Write> serde::ser::SerializeStruct for SerializerMap<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}
impl<'a, W: 'a + Write> serde::ser::SerializeSeq for SerializerTuple<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        <Self as serde::ser::SerializeTuple>::serialize_element(self, value)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeTuple>::end(self)
    }
}
impl<'a, W: 'a + Write> serde::ser::SerializeTuple for SerializerTuple<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        value.serialize(&mut Delayed {
            ser: self.ser,
            header: self.first.then_some(DelayedHeader::List { len: self.len }),
            is_list: true,
        })?;
        self.first = false;
        Ok(())
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: 'a + Write> serde::ser::SerializeTupleStruct for SerializerTuple<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.serialize_element(value)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a, W: 'a + Write> serde::ser::SerializeTupleVariant for SerializerTuple<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.serialize_element(value)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a, W: 'a + Write> Delayed<'a, W> {
    fn write_header(&mut self, tag: NbtTypeId) -> Result<()> {
        if let Some(header) = self.header.take() {
            write_header(&mut self.ser.writer, header, tag)?;
        }
        Ok(())
    }
}
impl<'a, W: 'a + Write> serde::ser::Serializer for &'a mut Delayed<'a, W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = SerializerTuple<'a, W>;

    type SerializeTuple = SerializerTuple<'a, W>;

    type SerializeTupleStruct = SerializerTuple<'a, W>;

    type SerializeTupleVariant = SerializerTuple<'a, W>;

    type SerializeMap = SerializerMap<'a, W>;

    type SerializeStruct = SerializerMap<'a, W>;

    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Byte)?;
        self.ser.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i16(self, v: i16) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u16(v as u16)
    }

    fn serialize_i32(self, v: i32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_i64(self, v: i64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Byte)?;
        match self.ser.writer.write_u8(v) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_u16(self, v: u16) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Short)?;
        match self.ser.writer.write_u16::<BigEndian>(v) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_u32(self, v: u32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Int)?;
        match self.ser.writer.write_u32::<BigEndian>(v) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }
    fn serialize_i128(self, v: i128) -> Result<()> {
        self.serialize_u128(v as u128)
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.write_header(NbtTypeId::IntArray)?;
        self.ser.writer.write_len(4)?;
        match self.ser.writer.write_all(&v.to_be_bytes()) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(())
        }
    }

    fn serialize_u64(self, v: u64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Long)?;
        match self.ser.writer.write_u64::<BigEndian>(v) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_f32(self, v: f32) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Float)?;
        match self.ser.writer.write_all(&v.to_be_bytes()) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_f64(self, v: f64) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Double)?;
        match self.ser.writer.write_all(&v.to_be_bytes()) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_char(self, v: char) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::Int)?;
        match self.ser.writer.write_all(&(v as u32).to_be_bytes()) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_str(self, v: &str) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::String)?;
        self.ser.writer.write_nbt_string(v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::List)?;
        self.ser.writer.write_tag(NbtTypeId::Byte)?;
        self.ser.writer.write_len(v.len())?;
        match self.ser.writer.write_all(v) {
            Err(_) => Err(Error::Message("failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_none(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        if self.is_list {
            Err(Error::Message("cannot serialize None a a list".to_string()))
        } else {
            Ok(())
        }
    }

    fn serialize_some<T>(self, value: &T) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Err(Error::Message("cannot serialize unit: ()".to_string()))
    }

    fn serialize_unit_struct(self, name: &'static str) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Err(Error::Message(format!("cannot serialize unit struct: {name}")))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write_header(NbtTypeId::String)?;
        self.ser.writer.write_nbt_string(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(Error::Message(format!("cannot serialize newtype variant")))
    }

    fn serialize_seq(self, len: Option<usize>) -> std::prelude::v1::Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or_else(|| Error::Message("sequeces must have known lengths".to_string()))?;
        self.serialize_tuple(len)
    }

    fn serialize_tuple(self, len: usize) -> std::prelude::v1::Result<Self::SerializeTuple, Self::Error> {
        self.write_header(NbtTypeId::List)?;
        if len == 0 {
            self.ser.writer.write_tag(NbtTypeId::List)?;
            if let Err(_) = self.ser.writer.write_u32::<BigEndian>(0) {
                return Err(Error::Message("Failed to write".to_string()));
            }
        }
        Ok(SerializerTuple {
            ser: self.ser,
            first: true,
            len,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> std::prelude::v1::Result<Self::SerializeMap, Self::Error> {
        Ok( SerializerMap { 
            ser: self.ser,
            key: None,
            header: self.header.take(),
            trailer: Some(NbtTypeId::End)
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Message("cannot serialize struct variant".to_string()))
    }
}

#[cfg(test)]
mod test{
    use serde::Serialize;

    use crate::{reader::NbtReader, traits::NbtRead, version::Java};

    #[test]
    fn test_ser() {
        let data = include_bytes!("../test_data/hello_world.nbt");
        let nbt_reader = NbtReader::new(data.to_vec());
        let nbt_data = Java::from_reader(nbt_reader).expect("This should have loaded");
    }
}
