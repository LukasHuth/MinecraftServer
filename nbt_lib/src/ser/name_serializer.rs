use std::io::Write;

use serde::{ser::Impossible, Serializer};

use crate::error::Error;

pub(super) struct NameSerializer<W: Write> {
    pub(super) name: W,
}
macro_rules! string_like_error {
    ($name:expr) => {
        Err(Error::Message(format!("field must be string-like, found {}",$name)))
    };
}

macro_rules! must_be_stringy {
    ($ser:ident($($t:ty),*) -> $res:ty) => {
        paste::paste!{
            #[inline]
            fn [< serialize_ $ser >](self, $(_: $t),*) -> crate::error::Result<$res> {
                string_like_error!(stringify!($ser))
            }
        }
    };

    ($ser:ident<T>($($t:ty),*) -> $res:ty) => {
        paste::paste!{
            #[inline]
            fn [< serialize_ $ser >]<T: ?Sized>(self, $(_: $t),*) -> crate::error::Result<$res> {
                string_like_error!(stringify!($ser))
            }
        }
    };

    ($ser:ident($($t:ty),*)) => {
        must_be_stringy!($ser($($t),*) -> ());
    };

    ($ser:ident<T>($($t:ty),*)) => {
        must_be_stringy!($ser<T>($($t),*) -> ());
    };
}
impl<'a, W: Write> Serializer for &mut NameSerializer<W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;

    type SerializeTuple = Impossible<(), Error>;

    type SerializeTupleStruct = Impossible<(), Error>;

    type SerializeTupleVariant = Impossible<(), Error>;

    type SerializeMap = Impossible<(), Error>;

    type SerializeStruct = Impossible<(), Error>;

    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        match self.name.write_all(&cesu8::to_java_cesu8(&v.to_string())) {
            Err(_) => Err(Error::Message("Failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        match self.name.write_all(&cesu8::to_java_cesu8(v)) {
            Err(_) => Err(Error::Message("Failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.name.write_all(v) {
            Err(_) => Err(Error::Message("Failed to write".to_string())),
            Ok(()) => Ok(()),
        }
    }

    must_be_stringy!(bool(bool));
    must_be_stringy!(i8(i8));
    must_be_stringy!(i16(i16));
    must_be_stringy!(i32(i32));
    must_be_stringy!(i64(i64));
    must_be_stringy!(u8(u8));
    must_be_stringy!(u16(u16));
    must_be_stringy!(u32(u32));
    must_be_stringy!(u64(u64));
    must_be_stringy!(f32(f32));
    must_be_stringy!(f64(f64));
    must_be_stringy!(none());
    must_be_stringy!(some<T>(&T));
    must_be_stringy!(unit());
    must_be_stringy!(unit_struct(&'static str));
    must_be_stringy!(unit_variant(&'static str, u32, &'static str));
    must_be_stringy!(newtype_struct<T>(&'static str, &T));
    must_be_stringy!(newtype_variant<T>(&'static str, u32, &'static str, &T));
    must_be_stringy!(seq(Option<usize>) -> Self::SerializeSeq);
    must_be_stringy!(tuple(usize) -> Self::SerializeTuple);
    must_be_stringy!(tuple_struct(&'static str, usize) -> Self::SerializeTupleStruct);
    must_be_stringy!(tuple_variant(&'static str, u32, &'static str,  usize) -> Self::SerializeTupleVariant);
    must_be_stringy!(map(Option<usize>) -> Self::SerializeMap);
    must_be_stringy!(struct(&'static str, usize) -> Self::SerializeStruct);
    must_be_stringy!(struct_variant(&'static str, u32, &'static str, usize) -> Self::SerializeStructVariant);
}
