//! Module for NBT deserialization
//!
//! The following code is copied and modified from fastnbt:
//! <https://github.com/owengage/fastnbt/blob/da6d919ac3916d2a951ee79451497f4981802ca9/fastnbt/src/value/de.rs>
//!
//! for which the license is MIT
use std::{borrow::Cow, collections::HashMap};

use serde::{de::{value::{BorrowedStrDeserializer, BytesDeserializer}, DeserializeSeed, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor}, forward_to_deserialize_any, Deserialize, Deserializer};
use serde_bytes::ByteBuf;

use crate::{error::Error, NbtValue};

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::{reader::NbtReader, traits::NbtRead, version::Java};
 
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Test {
        name: String,
    }
    #[test]
    fn test() {
        let test_nbt_data = include_bytes!("../../test_data/hello_world.nbt");
        let reader = NbtReader::new(test_nbt_data.to_vec());
        let nbt_data = Java::from_reader(reader).unwrap();
        let result = dbg!(Test::deserialize(&nbt_data)).expect("This should not fail, because the loaded file is a valid nbt file");
        let expected_result = Test { name: "Bananrama".to_string() };
        assert_eq!(result, expected_result);
    }
}

impl<'de> Deserialize<'de> for NbtValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct NbtValueVisitor;
        impl<'de> serde::de::Visitor<'de> for NbtValueVisitor {
            type Value = NbtValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("valid NBT")
            }
            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Byte(v))
            }
            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Short(v))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Int(v))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Long(v))
            }
            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Float(v))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::Double(v))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(NbtValue::String(v.to_owned()))
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>, {
                let mut v = Vec::<NbtValue>::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(element) = seq.next_element()? {
                    v.push(element);
                }
                Ok(NbtValue::List(v))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                match map.next_key_seed(KeyClassifier)? {
                    Some(KeyClass::Compound(first_key)) => {
                        let mut compound = HashMap::new();
                        compound.insert(first_key, map.next_value()?);
                        while let Some((key, value)) = map.next_entry()? {
                            compound.insert(key, value);
                        }
                        Ok(NbtValue::Compound(Some("".to_string()), compound))
                    }
                    Some(KeyClass::ByteArray) => {
                        let data = map.next_value::<ByteBuf>()?;
                        Ok(NbtValue::ByteArray(data.iter().map(|&e|e as i8).collect()))
                    }
                    Some(KeyClass::IntArray) => {
                        let data = map.next_value::<ByteBuf>()?;
                        if data.len() & 3 != 0 { return Err(serde::de::Error::custom("could not read the int array")); }
                        Ok(NbtValue::IntArray(data.chunks(4).map(|chunk|
                            i32::from_be_bytes(chunk.try_into().expect("This should never fail, because i check the size in the beginning"))
                        ).collect()))
                    }
                    Some(KeyClass::LongArray) => {
                        let data = map.next_value::<ByteBuf>()?;
                        if data.len() & 7 != 0 { return Err(serde::de::Error::custom("could not read the long array")); }
                        Ok(NbtValue::LongArray(data.chunks(8).map(|chunk|
                            i64::from_be_bytes(chunk.try_into().expect("This should never fail, because i check the size in the beginning"))
                        ).collect()))
                    }
                    None => Ok(NbtValue::Compound(None, Default::default()))
                }
            }
        }

        deserializer.deserialize_any(NbtValueVisitor)
    }
}
struct KeyClassifier;
enum KeyClass {
    Compound(String),
    ByteArray,
    IntArray,
    LongArray,
}
impl<'de> DeserializeSeed<'de> for KeyClassifier {
    type Value = KeyClass;

    fn deserialize<D>(self, deserializer: D) -> Result<KeyClass, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for KeyClassifier {
    type Value = KeyClass;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an nbt field string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "__byte_array" => Ok(KeyClass::ByteArray),
            "__int_array" => Ok(KeyClass::IntArray),
            "__long_array" => Ok(KeyClass::LongArray),
            _ => Ok(KeyClass::Compound(v)),
        }
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "__byte_array" => Ok(KeyClass::ByteArray),
            "__int_array" => Ok(KeyClass::IntArray),
            "__long_array" => Ok(KeyClass::LongArray),
            _ => Ok(KeyClass::Compound(v.to_string())),
        }
    }
}

macro_rules! deserialize_number {
    ($method:ident, $visit:ident, $primitive:ident, $variant:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self {
                NbtValue::$variant(v) => visitor.$visit(*v as $primitive),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    };
}

fn visit_list<'de, V>(list: &'de Vec<NbtValue>, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = list.len();
    let mut deserializer = SeqDeserializer::new(list);
    let seq = visitor.visit_seq(&mut deserializer)?;
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in list",
        ))
    }
}

fn visit_compound<'de, V>(
    compound: &'de HashMap<String, NbtValue>,
    visitor: V,
) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = compound.len();
    let mut deserializer = MapDeserializer::new(compound);
    let map = visitor.visit_map(&mut deserializer)?;
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(map)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in map",
        ))
    }
}

fn get_i128_value(de: &NbtValue) -> Result<i128, Error> {
    match de {
        NbtValue::IntArray(v) => {
            if v.len() != 4 {
                Err(Error::Message(format!(
                    "deserialize i128: expected IntArray of length 4, got length {}",
                    v.len()
                )))
            } else {
                Ok(v.iter()
                    .rev()
                    .flat_map(|n| (0..32).map(move |bit| n >> bit & 1))
                    .rev()
                    .fold(0, |acc, bit| acc << 1 | bit as i128))
            }
        }
        v => Err(Error::Message(format!(
            "deserialize i128: expected IntArray value {v:?}"
        ))),
    }
}

impl<'de> serde::Deserializer<'de> for &'de NbtValue {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match *self {
            NbtValue::Byte(val) => visitor.visit_i8(val),
            NbtValue::Short(val) => visitor.visit_i16(val),
            NbtValue::Int(val) => visitor.visit_i32(val),
            NbtValue::Long(val) => visitor.visit_i64(val),
            NbtValue::Float(val) => visitor.visit_f32(val),
            NbtValue::Double(val) => visitor.visit_f64(val),
            NbtValue::String(ref val) => visitor.visit_borrowed_str(val),
            NbtValue::ByteArray(_) => visitor.visit_map(ArrayAccess {
                token: "__byte_array",
                value: self,
            }),
            NbtValue::IntArray(_) => visitor.visit_map(ArrayAccess {
                token: "__int_array",
                value: self,
            }),
            NbtValue::LongArray(_) => visitor.visit_map(ArrayAccess {
                token: "__long_array",
                value: self,
            }),
            NbtValue::List(ref val) => visit_list(val, visitor),
            NbtValue::Compound(_,ref val) => visit_compound(val, visitor),
        }
    }

    deserialize_number!(deserialize_i8, visit_i8, i8, Byte);
    deserialize_number!(deserialize_i16, visit_i16, i16, Short);
    deserialize_number!(deserialize_i32, visit_i32, i32, Int);
    deserialize_number!(deserialize_i64, visit_i64, i64, Long);
    deserialize_number!(deserialize_u8, visit_u8, u8, Byte);
    deserialize_number!(deserialize_u16, visit_u16, u16, Short);
    deserialize_number!(deserialize_u32, visit_u32, u32, Int);
    deserialize_number!(deserialize_u64, visit_u64, u64, Long);
    deserialize_number!(deserialize_f32, visit_f32, f32, Float);
    deserialize_number!(deserialize_f64, visit_f64, f64, Double);

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(get_i128_value(self)?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(get_i128_value(self)? as u128)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self {
            NbtValue::Compound(_,value) => {
                let mut iter = value.iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(serde::de::Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ));
                    }
                };
                // enums are encoded in nbt as maps with a single key:value pair
                if iter.next().is_some() {
                    return Err(serde::de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(value))
            }
            NbtValue::String(variant) => (variant, None),
            other => {
                return Err(serde::de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            }
        };

        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::Byte(v) => visitor.visit_bool(v != &0),
            NbtValue::Short(v) => visitor.visit_bool(v != &0),
            NbtValue::Int(v) => visitor.visit_bool(v != &0),
            NbtValue::Long(v) => visitor.visit_bool(v != &0),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::Int(v) => match char::from_u32(*v as u32) {
                Some(v) => visitor.visit_char(v),
                None => Err(serde::de::Error::invalid_value(
                    self.unexpected(),
                    &"invalid character code",
                )),
            },
            NbtValue::String(ref v) => match v.chars().next() {
                Some(v) => visitor.visit_char(v),
                None => Err(serde::de::Error::invalid_value(
                    self.unexpected(),
                    &"string contains no character",
                )),
            },
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::String(v) => visitor.visit_borrowed_str(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::String(v) => visitor.visit_borrowed_str(v),
            NbtValue::List(v) => visit_list(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::List(v) => visit_list(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            NbtValue::List(v) => visit_list(v, visitor),
            NbtValue::Compound(_, v) => visit_compound(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

/// A struct for array access in NBT serialization
pub struct ArrayAccess<'de> {
    /// The name of the value
    pub token: &'static str,
    /// The value
    pub value: &'de NbtValue,
}
impl<'de> MapAccess<'de> for ArrayAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        seed.deserialize(BorrowedStrDeserializer::new(self.token))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let data: Vec<u8> = match self.value {
            NbtValue::ByteArray(v) => v.iter().map(|&e|e as u8).collect(),
            NbtValue::IntArray(v) => v.iter().map(|&e|e).map(i32::to_be_bytes).flatten().collect(),
            NbtValue::LongArray(v) => v.iter().map(|&e|e).map(i64::to_be_bytes).flatten().collect(),
            _ => unreachable!(),
        };
        let dz = BytesDeserializer::new(&data);
        seed.deserialize(dz)
    }
}

struct EnumDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de NbtValue>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

impl<'de> IntoDeserializer<'de, Error> for &'de NbtValue {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

struct VariantDeserializer<'de> {
    value: Option<&'de NbtValue>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(value) => serde::Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(NbtValue::List(v)) => {
                if v.is_empty() {
                    visitor.visit_unit()
                } else {
                    visit_list(v, visitor)
                }
            }
            Some(other) => Err(serde::de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(NbtValue::Compound(_, v)) => visit_compound(v, visitor),
            Some(other) => Err(serde::de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

struct SeqDeserializer<'de> {
    iter: std::slice::Iter<'de, NbtValue>,
}

impl<'de> SeqDeserializer<'de> {
    fn new(slice: &'de [NbtValue]) -> Self {
        SeqDeserializer { iter: slice.iter() }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapDeserializer<'de> {
    iter: <&'de HashMap<String, NbtValue> as IntoIterator>::IntoIter,
    value: Option<&'de NbtValue>,
}

impl<'de> MapDeserializer<'de> {
    fn new(map: &'de HashMap<String, NbtValue>) -> Self {
        MapDeserializer {
            iter: map.iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = MapKeyDeserializer {
                    key: Cow::Borrowed(&**key),
                };
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapKeyDeserializer<'de> {
    key: Cow<'de, str>,
}

macro_rules! deserialize_integer_key {
    ($method:ident => $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            match (self.key.parse(), self.key) {
                (Ok(integer), _) => visitor.$visit(integer),
                (Err(_), Cow::Borrowed(s)) => visitor.visit_borrowed_str(s),
                (Err(_), Cow::Owned(s)) => visitor.visit_string(s),
            }
        }
    };
}

impl<'de> serde::Deserializer<'de> for MapKeyDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        BorrowedCowStrDeserializer::new(self.key).deserialize_any(visitor)
    }

    deserialize_integer_key!(deserialize_i8 => visit_i8);
    deserialize_integer_key!(deserialize_i16 => visit_i16);
    deserialize_integer_key!(deserialize_i32 => visit_i32);
    deserialize_integer_key!(deserialize_i64 => visit_i64);
    deserialize_integer_key!(deserialize_u8 => visit_u8);
    deserialize_integer_key!(deserialize_u16 => visit_u16);
    deserialize_integer_key!(deserialize_u32 => visit_u32);
    deserialize_integer_key!(deserialize_u64 => visit_u64);

    deserialize_integer_key!(deserialize_i128 => visit_i128);
    deserialize_integer_key!(deserialize_u128 => visit_u128);

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        // Map keys cannot be null.
        visitor.visit_some(self)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.key
            .into_deserializer()
            .deserialize_enum(name, variants, visitor)
    }

    forward_to_deserialize_any! {
        bool f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl NbtValue {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected(&self) -> Unexpected {
        match self {
            NbtValue::Byte(v) => Unexpected::Signed(*v as i64),
            NbtValue::Short(v) => Unexpected::Signed(*v as i64),
            NbtValue::Int(v) => Unexpected::Signed(*v as i64),
            NbtValue::Long(v) => Unexpected::Signed(*v),
            NbtValue::Float(v) => Unexpected::Float(*v as f64),
            NbtValue::Double(v) => Unexpected::Float(*v),
            NbtValue::String(v) => Unexpected::Str(v),
            NbtValue::ByteArray(_) => Unexpected::Seq,
            NbtValue::IntArray(_) => Unexpected::Seq,
            NbtValue::LongArray(_) => Unexpected::Seq,
            NbtValue::List(_) => Unexpected::Seq,
            NbtValue::Compound(_,_) => Unexpected::Map,
        }
    }
}

struct BorrowedCowStrDeserializer<'de> {
    value: Cow<'de, str>,
}

impl<'de> BorrowedCowStrDeserializer<'de> {
    fn new(value: Cow<'de, str>) -> Self {
        BorrowedCowStrDeserializer { value }
    }
}

impl<'de> Deserializer<'de> for BorrowedCowStrDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
            Cow::Owned(string) => visitor.visit_string(string),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl<'de> EnumAccess<'de> for BorrowedCowStrDeserializer<'de> {
    type Error = Error;
    type Variant = UnitOnly;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Error>
    where
        T: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self)?;
        Ok((value, UnitOnly))
    }
}

struct UnitOnly;

impl<'de> VariantAccess<'de> for UnitOnly {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}
