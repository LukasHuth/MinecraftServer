//! This is a Module for deserializing [`NbtValue`]'s
//!
//! The following code is copied and modified from fastnbt:
//! <https://github.com/owengage/fastnbt/blob/da6d919ac3916d2a951ee79451497f4981802ca9/fastnbt/src/ser/serializer.rs>
//!
//! for which the license is MIT
//!
//! [`NbtValue`]: `crate::NbtValue`
//!
//! # Example
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use nbt_lib::de::from_bytes;
//! #[derive(Debug, Deserialize, PartialEq, Eq)]
//! struct Test {
//!     name: String
//! }
//! let data = include_bytes!("../../test_data/hello_world.nbt");
//! let test_struct = Test { name: "Bananrama".to_string() };
//! let test_data: Test = from_bytes(data).expect("This should not fail");
//! assert_eq!(test_data, test_struct);
//! ```

/// Converts raw nbt bytes directly into an struct
pub fn from_bytes<'de, T: Deserialize<'static>>(v: &'static [u8]) -> Result<T> {
    let data: Slice = Slice { data: v };
    let mut test = Deserializer::new(data, DeOpts { max_seq_len: usize::MAX });
    T::deserialize(&mut test)
}

#[cfg(test)]
mod test {
}

use std::io::Read;

use serde::{de::{self, value::{BorrowedBytesDeserializer, BorrowedStrDeserializer, BytesDeserializer}}, forward_to_deserialize_any, Deserialize };

use crate::{error::{Error, Result}, NbtTypeId};

use self::input::{Input, Reference, Slice};


/// A struct for the desierialization options
pub struct DeOpts {
    max_seq_len: usize
}

/// A struct for deserializing raw NBT data into other datastructs
pub struct Deserializer<In> {
    input: In,
    scratch: Vec<u8>,
    seen_root: bool,
    opts: DeOpts,
    root_name: String,
}

mod input;

impl<'de, In> Deserializer<In> where In: input::Input<'de> {
    /// creates a new instance
    pub fn new(input: In, opts: DeOpts) -> Self {
        Self {
            input,
            scratch: Vec::new(),
            seen_root: false,
            opts,
            root_name: String::new()
        }
    }
}
impl<'a> Deserializer<input::Slice<'a>> {
    /// creates a deserializer from raw NBT bytes
    pub fn from_bytes(bytes: &'a [u8], opts: DeOpts) -> Self {
        Deserializer::new(input::Slice { data: bytes }, opts)
    }
}

impl<R: Read> Deserializer<input::Reader<R>> {
    /// creates a deserializer from a reader containing raw NBT bytes
    pub fn from_reader(reader: R, opts: DeOpts) -> Self {
        Deserializer::new(input::Reader { reader }, opts)
    }
}

impl<'de, 'a, In: Input<'de>> de::Deserializer<'de> for &'a mut Deserializer<In> {
    type Error = Error;

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit unit_struct seq tuple tuple_struct
        identifier ignored_any bytes enum newtype_struct byte_buf option
    }
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        if !self.seen_root {
            let peek = self.input.consume_tag()?;
            match peek {
                NbtTypeId::Compound => {
                    let mut root_name = Vec::new();
                    self.input.consume_str(&mut root_name)?;
                    self.root_name = String::from_utf8(root_name).unwrap_or(String::new());
                }
                _ => return Err(Error::no_root_compound()),
            }
            self.seen_root = true;
        }

        visitor.visit_map(MapAccess::new(self))
    }

    #[inline]
    fn deserialize_struct<V>(self,_name: &'static str,_fields: &'static [&'static str],visitor: V,) -> Result<V::Value>
where
        V: de::Visitor<'de> {
        self.deserialize_map(visitor)
    }
}

struct MapAccess<'a, In: 'a> {
    de: &'a mut Deserializer<In>,
    tag: NbtTypeId,
}
impl<'a, In: 'a> MapAccess<'a, In> {
    pub fn new(de: &'a mut Deserializer<In>) -> Self {
        Self { de, tag: NbtTypeId::End }
    }
}
impl<'de, 'a, In: Input<'de> + 'a> de::MapAccess<'de> for MapAccess<'a, In> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de> {
        self.tag = self.de.input.consume_tag()?;
        if self.tag == NbtTypeId::End { return Ok(None) }
        seed.deserialize(MapKey { de: &mut *self.de }).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de> {
        seed.deserialize(AnonymousValue {
            tag: self.tag,
            de: &mut *self.de,
            last_hint: Hint::None,
        })
    }
}

struct MapKey<'a, In> {
    de: &'a mut Deserializer<In>,
}

fn arr_check(key: &str) -> crate::error::Result<&str> {
    if key.starts_with("__") && (key == "__byte_array" || key == "__int_array" || key == "__long_array") {
        return Err(Error::Message("compound using special array tokens".to_string()))
    }
    Ok(key)
}

impl<'de, 'a, R: Input<'de>> de::Deserializer<'de> for MapKey<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        match self.de.input.consume_str(&mut self.de.scratch)? {
            input::Reference::Borrowed(s) => visitor.visit_borrowed_str(arr_check(s)?),
            input::Reference::Copied(s) => visitor.visit_str(arr_check(s)?),
        }
    }
    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit unit_struct seq tuple tuple_struct map
        struct identifier ignored_any bytes enum newtype_struct byte_buf option
    }
}

enum Hint {
    None,
    Seq
}

struct AnonymousValue<'a, In> {
    tag: NbtTypeId,
    last_hint: Hint,
    de: &'a mut Deserializer<In>
}

impl<'de, 'a, In> de::Deserializer<'de> for AnonymousValue<'a, In> where In: Input<'de> {
    type Error = Error;

    forward_to_deserialize_any!(u8 u16 u32 u64 i8 i16 i32 i64 f32
    f64 str string struct tuple map identifier char);

    fn deserialize_any<V>(mut self, v: V) -> Result<V::Value>
where
        V: de::Visitor<'de> {
        let last_hint = self.last_hint;
        self.last_hint = Hint::None;

        match self.tag {
            NbtTypeId::End => Err(Error::Message("expected value, found end tag".into())),
            NbtTypeId::Byte => v.visit_i8(self.de.input.consume_byte()? as i8),
            NbtTypeId::Short => v.visit_i16(self.de.input.consume_i16()?),
            NbtTypeId::Int => v.visit_i32(self.de.input.consume_i32()?),
            NbtTypeId::Long => v.visit_i64(self.de.input.consume_i64()?),
            NbtTypeId::Float => v.visit_f32(self.de.input.consume_f32()?),
            NbtTypeId::Double => v.visit_f64(self.de.input.consume_f64()?),
            NbtTypeId::String => match self.de.input.consume_str(&mut self.de.scratch)? {
                Reference::Borrowed(s) => v.visit_borrowed_str(s),
                Reference::Copied(s) => v.visit_str(s),
            },
            NbtTypeId::List => {
                let tag = self.de.input.consume_tag()?;
                let remaining = self.de.input.consume_i32()? as usize;

                // End values have no payload. An end tag on it's own is the payload
                // of an empty compound. A logical interpretation is that this could
                // be a list of zero-sized units, but this mean an easy short
                // malicious payload of a massive list taking up lots of memory (as
                // the Value type's unit variant would not be zero sized.
                //
                // Some old chunks store empty lists as as 'list of end', so if the
                // size is zero we let it slide.
                if tag == NbtTypeId::End && remaining != 0 {
                    return Err(Error::Message(
                        "unexpected list of type 'end', which is not supported".into(),
                    ));
                }

                if remaining > self.de.opts.max_seq_len {
                    return Err(Error::Message(format!(
                        "size ({}) greater than max sequence length ({})",
                        remaining, self.de.opts.max_seq_len,
                    )));
                }

                v.visit_seq(ListAccess {
                    de: self.de,
                    tag,
                    remaining,
                })
            }
            NbtTypeId::Compound => v.visit_map(MapAccess::new(self.de)),
            NbtTypeId::ByteArray => {
                if let Hint::Seq = last_hint {
                    return Err(Error::Message(format!("expected NBT Array, found seq: use ByteArray, IntArray or LongArray types")));
                }
                let len = self.de.input.consume_i32()? as usize;
                v.visit_map(ArrayWrapperAccess::bytes(self.de, len)?)
            }
            NbtTypeId::IntArray => {
                if let Hint::Seq = last_hint {
                    return Err(Error::Message(format!("expected NBT Array, found seq: use ByteArray, IntArray or LongArray types")));
                }
                let len = self.de.input.consume_i32()? as usize;
                v.visit_map(ArrayWrapperAccess::ints(self.de, len)?)
            }
            NbtTypeId::LongArray => {
                if let Hint::Seq = last_hint {
                    return Err(Error::Message(format!("expected NBT Array, found seq: use ByteArray, IntArray or LongArray types")));
                }
                let len = self.de.input.consume_i32()? as usize;
                v.visit_map(ArrayWrapperAccess::longs(self.de, len)?)
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
where
        V: de::Visitor<'de> {
        let consume_visit =
            |de: &mut Deserializer<In>, len: usize, el_size| match de.input.consume_bytes(
                len.checked_mul(el_size)
                    .ok_or_else(|| Error::Message("overflow deserializing bytes".to_string()))?,
                &mut de.scratch,
            )? {
                Reference::Borrowed(bs) => visitor.visit_borrowed_bytes(bs),
                Reference::Copied(bs) => visitor.visit_bytes(bs),
            };

        match self.tag {
            NbtTypeId::String => {
                let len = self.de.input.consume_i16()? as usize;
                consume_visit(self.de, len, 1)
            }
            NbtTypeId::List => {
                let tag = self.de.input.consume_tag()?;
                let remaining = self.de.input.consume_i32()? as usize;

                match tag {
                    NbtTypeId::Byte => consume_visit(self.de, remaining, std::mem::size_of::<i8>()),
                    NbtTypeId::Short => consume_visit(self.de, remaining, std::mem::size_of::<i16>()),
                    NbtTypeId::Int => consume_visit(self.de, remaining, std::mem::size_of::<i32>()),
                    NbtTypeId::Long => consume_visit(self.de, remaining, std::mem::size_of::<i64>()),
                    _ => Err(Error::Message(format!(
                        "cannot convert list of {} to bytes",
                        tag
                    ))),
                }
            }
            NbtTypeId::ByteArray => {
                let remaining = self.de.input.consume_i32()? as usize;
                consume_visit(self.de, remaining, std::mem::size_of::<i8>())
            }
            NbtTypeId::LongArray => {
                let remaining = self.de.input.consume_i32()? as usize;
                consume_visit(self.de, remaining, std::mem::size_of::<i64>())
            }
            _ => Err(Error::Message(format!(
                "cannot convert {} to bytes",
                self.tag
            ))),
        }
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
where
        V: de::Visitor<'de> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
where
        V: de::Visitor<'de> {
        match self.tag {
            NbtTypeId::Byte => visitor.visit_bool(self.de.input.consume_byte()? != 0),
            NbtTypeId::Short => visitor.visit_bool(self.de.input.consume_i16()? != 0),
            NbtTypeId::Int => visitor.visit_bool(self.de.input.consume_i32()? != 0),
            NbtTypeId::Long => visitor.visit_bool(self.de.input.consume_i64()? != 0),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        self.de.input.ignore_value(self.tag)?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        self.last_hint = Hint::Seq;
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        visitor.visit_enum(UnitVariantAccess {
            de: AnonymousValue {
                tag: self.tag,
                de: self.de,
                last_hint: Hint::None,
            }
        })
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        self.deserialize_seq(visitor)
    }
    #[inline]
    fn deserialize_i128<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>, {
        visitor.visit_i128(get_i128_value(&mut self)?)
    }
    #[inline]
    fn deserialize_u128<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>, {
        visitor.visit_u128(get_i128_value(&mut self)? as u128)
    }
}

struct ListAccess<'a, In: 'a> {
    de: &'a mut Deserializer<In>,
    tag: NbtTypeId,
    remaining: usize,
}

impl<'de, 'a, In: Input<'de> + 'a> de::SeqAccess<'de> for ListAccess<'a, In> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de> {
        if self.remaining == 0 { return Ok(None) }
        self.remaining -= 1;
        seed.deserialize(AnonymousValue {
            de: &mut *self.de,
            last_hint: Hint::None,
            tag: self.tag,
        }).map(Some)
    }
}

struct UnitVariantAccess<'a, In: 'a> {
    de: AnonymousValue<'a, In>
}

impl<'de, 'a, In: Input<'de> + 'a> de::EnumAccess<'de> for UnitVariantAccess<'a, In> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de> {
        let variant = seed.deserialize(AnonymousValue {
            de: &mut *self.de.de,
            last_hint: Hint::None,
            tag: self.de.tag
        })?;
        Ok((variant, self))
    }
}

impl<'de, 'a, In: Input<'de> + 'a> de::VariantAccess<'de> for UnitVariantAccess<'a, In> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de> {
        Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"newtype variant"))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"tuple variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de> {
        Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"struct variant"))
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Unread, 
    Read
}

pub(crate) struct ArrayWrapperAccess<'a, In: 'a> {
    de: &'a mut Deserializer<In>,
    token: &'static str,
    bytes_size: usize,
    state: State,
}

impl<'a, In: 'a> ArrayWrapperAccess<'a, In> {
    pub(crate) fn bytes(de: &'a mut Deserializer<In>, size: usize) -> Result<Self> {
        Ok(Self {
            de,
            bytes_size: size
                .checked_mul(1)
                .ok_or_else(|| Error::Message("nbt array too large".to_string()))?,
            token: "__byte_array",
            state: State::Unread,
        })
    }

    pub(crate) fn ints(de: &'a mut Deserializer<In>, size: usize) -> Result<Self> {
        Ok(Self {
            de,
            bytes_size: size
                .checked_mul(4)
                .ok_or_else(|| Error::Message("nbt array too large".to_string()))?,
            token: "__int_array",
            state: State::Unread,
        })
    }

    pub(crate) fn longs(de: &'a mut Deserializer<In>, size: usize) -> Result<Self> {
        Ok(Self {
            de,
            bytes_size: size
                .checked_mul(8)
                .ok_or_else(|| Error::Message("nbt array too large".to_string()))?,
            token: "__long_array",
            state: State::Unread,
        })
    }
}

impl<'de, 'a, In: Input<'de> + 'a> de::MapAccess<'de> for ArrayWrapperAccess<'a, In> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::prelude::v1::Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de> {
        if self.state == State::Read { return Ok(None) }
        self.state = State::Read;
        seed.deserialize(BorrowedStrDeserializer::new(self.token)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de> {
        let data = self.de.input.consume_bytes(self.bytes_size, &mut self.de.scratch)?;
        match data {
            Reference::Borrowed(bs) => seed.deserialize(BorrowedBytesDeserializer::new(bs)),
            Reference::Copied(bs) => seed.deserialize(BytesDeserializer::new(bs))
        }
    }
}

fn get_i128_value<'de, In: Input<'de>>(de: &mut AnonymousValue<In>) -> Result<i128> {
    let tag = de.tag;

    match tag {
        NbtTypeId::IntArray => {
            let size = de.de.input.consume_i32()? as usize;

            let size = size.checked_mul(4).ok_or_else(|| Error::Message("nbt array too large".to_string()))?;

            let bs = de.de.input.consume_bytes(size, &mut de.de.scratch)?;
            let bs = bs.as_ref();

            match bs.try_into() {
                Ok(bs) => Ok(i128::from_be_bytes(bs)),
                Err(_) => Err(Error::Message(format!("deserialize i128: expected IntArray of length 4 with 16 bytes, found {} bytes", bs.len())))
            }
        }
        _ => Err(Error::ExpectedIntArray)
    }
}

