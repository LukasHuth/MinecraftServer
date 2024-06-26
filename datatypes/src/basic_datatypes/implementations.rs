use crate::{GetU64, ImportantFunctions as _};
use binary_utils::{read_byte, write_bytes, DataReader, DataWriter, Error, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

use super::*;

impl DataWriter for Boolean {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        Byte::new(if self.0 { 0x01 } else { 0x00 })
            .write(writer)
            .await
    }
}
impl DataReader for Boolean {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_i8().await {
            return Ok(Self(if data == 0 { false } else { true }));
        }
        Err(Error::InvalidStructure)
    }
}
impl DataReader for Byte {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_i8().await {
            return Ok(Self(data));
        }
        Err(Error::InvalidStructure)
    }
}
impl DataReader for UnsignedByte {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_u8().await {
            return Ok(Self(data));
        }
        Err(Error::InvalidStructure)
    }
}
impl DataReader for Long {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self>
    where
        Self: Sized,
    {
        let mut data = 0;
        for _ in 0..8 {
            data <<= 8;
            data |= read_byte(reader, line!(), file!()).await? as u64;
        }
        Ok(Self(data as i64))
    }
}
impl DataReader for Int {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self>
    where
        Self: Sized,
    {
        let mut data = 0;
        for _ in 0..4 {
            data <<= 8;
            data |= read_byte(reader, line!(), file!()).await? as u32;
        }
        Ok(Self(data as i32))
    }
}
impl DataReader for UnsignedShort {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self>
    where
        Self: Sized,
    {
        let data = ((read_byte(reader, line!(), file!()).await? as u16) << 8)
            | read_byte(reader, line!(), file!()).await? as u16;
        Ok(Self(data))
    }
}
macro_rules! implement_get_u64 {
    ($($i:ident),*) => {
        $(
        impl GetU64 for $i {
            fn get_u64(&self) -> u64 {
                self.0 as u64
            }
        }
        )*
    };
}
macro_rules! implement_data_writer {
    ($($i:ident),*) => {
        $(
        impl DataWriter for $i {
            async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
                write_bytes(writer, self.0.to_be_bytes().as_slice()).await
            }
        }
        )*
    };
}
implement_get_u64!(UnsignedByte, Byte, UnsignedShort, Short, Int, Long);
implement_data_writer!(UnsignedByte, Byte, UnsignedShort, Short, Int, Long, Float, Double);
impl DataReader for Float {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let mut data = [0; 4];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(f32::from_be_bytes(data))),
            Err(_) => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
impl DataReader for Double {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let mut data = [0; 8];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(f64::from_be_bytes(data))),
            Err(_) => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
