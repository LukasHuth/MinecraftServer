use std::io::Read;

use serde::Serialize;

use crate::{ImportantEnumTrait, errors::{Result, Error}, utils::{read_byte, DataReader, read_utf8_char, DataWriter, write_bytes, ListDataReader}};

pub mod important_enums;

pub struct Boolean(bool);
pub struct Byte(i8);
pub struct UnsignedByte(u8);
pub struct Short(i16);
#[derive(Debug)]
pub struct UnsignedShort(pub u16);
pub struct Int(i32);
pub struct Long(i64);
pub struct Float(f32);
pub struct Double(f64);
#[derive(Debug)]
pub struct String(pub(crate) std::string::String);
pub struct TextComponent(fastnbt::Value);
pub struct JSONTextComponent(String); // TODO: As JSON;
pub struct Identifier(String);
#[derive(Debug)]
pub struct VarInt(pub(crate) i32);
pub struct VarLong(i64);
pub struct EntityMetadata(entmet_lib::EntityMetadata);
pub struct Slot(slot_lib::Slot);
pub struct NBT(fastnbt::Value);
pub struct Position(i32, i32, i16);
pub struct Angle(u8);
pub struct UUID(u128);
pub struct BitSet(Vec<u64>);
pub struct FixedBitSet<const S: usize>([u8; S]); // INFO: S = ceil(size / 8)
pub struct Array<T>(Vec<T>) where T: DataReader + DataWriter;
#[derive(Debug)]
pub struct Enum<T, S>(pub(crate) T, pub(crate) S) where T: ImportantEnumTrait, S: DataReader + GetU64;
pub struct ByteArray(Vec<u8>);
impl<T> From<T> for JSONTextComponent where T: Serialize {
    fn from(value: T) -> Self {
        Self(String::new(serde_json::to_string(&value).unwrap_or(std::string::String::new())))
    }
}
impl DataWriter for JSONTextComponent {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        self.0.write(writer)?;
        Ok(())
    }
}
impl GetU64 for VarInt {
    fn get_u64(&self) -> u64 {
        self.0 as u64
    }
}
impl<T> ListDataReader for Array<T> where T: DataReader + DataWriter{
    fn read_list(reader: &mut impl Read, length: usize) -> Result<Self> where Self: Sized {
        let mut data = Vec::new();
        for _ in 0..length {
            data.push(T::read(reader)?);
        }
        Ok(Self(data))
    }
}
impl<T> DataWriter for Array<T> where T: DataReader + DataWriter{
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        for e in self.0.iter() {
            e.write(writer)?;
        }
        Ok(())
    }
}
impl ListDataReader for ByteArray {
    fn read_list(reader: &mut impl Read, length: usize) -> Result<Self> where Self: Sized {
        let mut data = vec![0; length];
        match reader.read_exact(&mut data) {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes.into(),
        }
    }
}
impl DataWriter for ByteArray {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        match writer.write_all(&self.0) {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl<const S: usize> DataReader for FixedBitSet<S> {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let mut data = [0u8; S];
        match reader.read_exact(&mut data) {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes.into(),
        }
    }
}
impl VarInt {
    pub fn new(data: i32) -> Self {
        Self(data)
    }
}
impl Long {
    pub fn new(data: i64) -> Self {
        Self(data)
    }
}
impl String {
    pub fn new(str: std::string::String) -> Self {
        Self(str)
    }
}
impl DataReader for VarInt {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let mut data: i32 = 0;
        loop {
            let current = read_byte(reader)?;
            data <<= 7;
            data |= (current & 0x7F) as i32;
            if current < 0x80 { break; }
        }
        Ok(Self(data))
    }
}
impl DataWriter for VarInt {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        let mut data = self.0;
        loop {
            let mut current = ((data & 0x7F) as u8) | 0x80;
            data >>= 7;
            if data == 0 {
                current &= 0x7F;
                return write_bytes(writer, &[current]);
            }
            write_bytes(writer, &[current])?;
        }
    }
}
impl DataReader for Long {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let mut data = 0;
        for _ in 0..8 {
            data <<= 8;
            data |= read_byte(reader)? as u64;
        }
        Ok(Self(data as i64))
    }
}
impl DataWriter for Long {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        let mut offset = 64 - 8;
        for _ in 0..8 {
            write_bytes(writer, &[((self.0 >> offset) & 0xFF) as u8])?;
            offset -= 8;
        }
        Ok(())
    }
}
impl DataReader for String {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let length = VarInt::read(reader)?;
        let mut chars = Vec::new();
        for _ in 0..length.0 {
            chars.push(read_utf8_char(reader)?);
        }
        let data: std::string::String = chars.iter().collect();
        Ok(Self(data))
    }
}
impl DataWriter for String {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<()> {
        let bytes = self.0.as_bytes();
        let length = VarInt::new(bytes.len() as i32);
        length.write(writer)?;
        write_bytes(writer, bytes)
    }
}
impl DataReader for UnsignedShort {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let data = ((read_byte(reader)? as u16) << 8) | read_byte(reader)? as u16;
        Ok(Self(data))
    }
}
pub trait GetU64 { fn get_u64(&self) -> u64; }
impl<T, S> DataReader for Enum<T, S> where S: DataReader + GetU64, T: ImportantEnumTrait {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized {
        let original_value = S::read(reader)?;
        let value = T::new(original_value.get_u64())?;
        Ok(Self(value, original_value))
    }
}
