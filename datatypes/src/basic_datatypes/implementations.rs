use binary_utils::{DataWriter, Result, Error, DataReader, read_byte, write_bytes};
use tokio::io::{AsyncWrite, AsyncWriteExt as _, AsyncRead, AsyncReadExt};
use super::*;


impl DataWriter for Boolean {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&[if self.0 { 0x01 } else { 0x00 }]).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl DataReader for Boolean {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_i8().await {
            return Ok(Self(if data == 0 { false } else { true }))
        }
        Err(Error::InvalidStructure)
    }
}
impl DataWriter for Byte {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&[self.0 as u8]).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl DataReader for Byte {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_i8().await {
            return Ok(Self(data))
        }
        Err(Error::InvalidStructure)
    }
}
impl DataReader for UnsignedByte {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        if let Ok(data) = reader.read_u8().await {
            return Ok(Self(data))
        }
        Err(Error::InvalidStructure)
    }
}
impl DataWriter for UnsignedByte {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&[self.0]).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
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
impl DataReader for UnsignedShort {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let data = ((read_byte(reader, line!(), file!()).await? as u16) << 8) | read_byte(reader, line!(), file!()).await? as u16;
        Ok(Self(data))
    }
}
