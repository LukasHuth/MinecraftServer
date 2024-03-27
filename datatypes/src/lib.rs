use binary_utils::{DataWriter, Error, Result, DataReader, write_bytes, ListDataReader, read_byte, read_utf8_char};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt as _, AsyncRead, AsyncReadExt as _};
pub trait ImportantEnumTrait : Sized {
    fn new(data: u64) -> Result<Self>;
}
pub struct Boolean(bool);
pub struct Byte(i8);
pub struct UnsignedByte(u8);
pub struct Short(i16);
#[derive(Debug)]
pub struct UnsignedShort(u16);
pub struct Int(i32);
pub struct Long(i64);
pub struct Float(f32);
pub struct Double(f64);
#[derive(Clone, Debug)]
pub struct String(std::string::String);
pub struct JSONTextComponent(String); // TODO: As JSON;
pub struct Identifier(String);
#[derive(Debug)]
pub struct VarInt(i32);
pub struct VarLong(i64);
pub struct Position(i32, i32, i16);
pub struct Angle(u8);
pub struct UUID(u128);
pub struct BitSet(Vec<u64>);
pub struct FixedBitSet<const S: usize>([u8; S]); // INFO: S = ceil(size / 8)
pub struct Array<T>(Vec<T>) where T: DataReader + DataWriter;
#[derive(Debug)]
pub struct Enum<T, S>(pub(crate) T, pub(crate) S) where T: ImportantEnumTrait, S: DataReader + GetU64;
pub struct ByteArray(Vec<u8>);
pub trait ImportantFunctions {
    type InputType;
    type ReturnType;
    fn new(data: Self::InputType) -> Self;
    fn get_value(&self) -> Self::ReturnType;
}
impl DataWriter for UUID {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        write_bytes(writer, &self.0.to_be_bytes()).await?;
        Ok(())
    }
}
impl DataReader for UUID {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let l0 = Long::read(reader).await?.get_value();
        let l1 = Long::read(reader).await?.get_value();
        let data = ((l0 as u128) << 64) | l1 as u128;
        Ok(Self(data))
    }
}
impl<T> From<T> for JSONTextComponent where T: Serialize {
    fn from(value: T) -> Self {
        Self(String::new(serde_json::to_string(&value).unwrap_or(std::string::String::new())))
    }
}
impl DataWriter for JSONTextComponent {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        self.0.write(writer).await?;
        Ok(())
    }
}
impl GetU64 for VarInt {
    fn get_u64(&self) -> u64 {
        self.0 as u64
    }
}
impl DataWriter for Boolean {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&[if self.0 { 0x01 } else { 0x00 }]).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl<T> ListDataReader for Array<T> where T: DataReader + DataWriter{
    async fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> Result<Self> where Self: Sized {
        let mut data = Vec::new();
        for _ in 0..length {
            data.push(T::read(reader).await?);
        }
        Ok(Self(data))
    }
}
impl<T> DataWriter for Array<T> where T: DataReader + DataWriter{
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        for e in self.0.iter() {
            e.write(writer).await?;
        }
        Ok(())
    }
}
impl ListDataReader for ByteArray {
    async fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> Result<Self> where Self: Sized {
        let mut data = vec![0; length];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into(),
        }
    }
}
impl DataWriter for ByteArray {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&self.0).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl<const S: usize> DataReader for FixedBitSet<S> {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let mut data = [0u8; S];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into(),
        }
    }
}
impl ImportantFunctions for VarInt {
    type InputType = i32;

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for UUID {
    type InputType = u128;

    type ReturnType = u128;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for ByteArray {
    type InputType = Vec<u8>;

    type ReturnType = Vec<u8>;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl ImportantFunctions for Long {
    type InputType = i64;

    type ReturnType = i64;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for String {
    type InputType = std::string::String;

    type ReturnType = std::string::String;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl ImportantFunctions for Boolean {
    type InputType = bool;

    type ReturnType = bool;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl DataReader for VarInt {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let mut data: i32 = 0;
        loop {
            let current = read_byte(reader, line!(), file!()).await?;
            data <<= 7;
            data |= (current & 0x7F) as i32;
            if current < 0x80 { break; }
        }
        Ok(Self(data))
    }
}
impl DataWriter for VarInt {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        let mut data = self.0;
        loop {
            let mut current = ((data & 0x7F) as u8) | 0x80;
            data >>= 7;
            if data == 0 {
                current &= 0x7F;
                return write_bytes(writer, &[current]).await;
            }
            write_bytes(writer, &[current]).await?;
        }
    }
}
impl DataReader for Long {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let mut data = 0;
        for _ in 0..8 {
            data <<= 8;
            data |= read_byte(reader, line!(), file!()).await? as u64;
        }
        Ok(Self(data as i64))
    }
}
impl DataWriter for Long {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        let mut offset = 64 - 8;
        for _ in 0..8 {
            write_bytes(writer, &[((self.0 >> offset) & 0xFF) as u8]).await?;
            offset -= 8;
        }
        Ok(())
    }
}
impl DataReader for String {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let length = VarInt::read(reader).await?;
        let mut chars = Vec::new();
        for _ in 0..length.0 {
            chars.push(read_utf8_char(reader, line!(), file!()).await?);
        }
        let data: std::string::String = chars.iter().collect();
        Ok(Self(data))
    }
}
impl DataWriter for String {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        let bytes = self.0.as_bytes();
        let length = VarInt::new(bytes.len() as i32);
        length.write(writer).await?;
        write_bytes(writer, bytes).await
    }
}
impl DataReader for UnsignedShort {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let data = ((read_byte(reader, line!(), file!()).await? as u16) << 8) | read_byte(reader, line!(), file!()).await? as u16;
        Ok(Self(data))
    }
}
pub trait GetU64 { fn get_u64(&self) -> u64; }
impl<T, S> DataReader for Enum<T, S> where S: DataReader + GetU64, T: ImportantEnumTrait {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let original_value = S::read(reader).await?;
        let value = T::new(original_value.get_u64())?;
        Ok(Self(value, original_value))
    }
}
impl<T, S> ImportantFunctions for Enum<T, S> where S: DataReader + GetU64, T: ImportantEnumTrait + Clone {
    type InputType = (T, S);

    type ReturnType = T;

    fn new(data: Self::InputType) -> Self {
        Self(data.0, data.1)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
