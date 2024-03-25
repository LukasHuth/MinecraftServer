use std::future::Future;
use tokio::io::{AsyncWrite, AsyncRead, AsyncWriteExt as _, AsyncReadExt as _};

use crate::errors::{Result, Error};

pub(crate) async fn read_byte(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<u8> {
    let mut data = [0u8; 1];
    match reader.read_exact(&mut data).await {
        Ok(_) => Ok(data[0]),
        Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file, line)).into(),
    }
}
pub(crate) async fn consume_utf16be_char(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<()> {
    read_byte(reader, line, file).await?;
    read_byte(reader, line, file).await?;
    Ok(())
}
pub(crate) async fn write_bytes(writer: &mut (impl AsyncWrite + Unpin), bytes: &[u8]) -> Result<()> {
    match writer.write_all(bytes).await {
        Ok(_) => Ok(()),
        Err(_) => Error::FailedToWrite.into(),
    }
}
pub(crate) async fn read_utf8_char(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<char> {
    let current = read_byte(reader, line, file).await? as u8;
    match current {
        0b0000_0000..=0b0111_1111 => 
            match char::from_u32(current as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1100_0000..=0b1101_1111 => 
            match char::from_u32(((current as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1110_0000..=0b1110_1111 => 
            match char::from_u32(((current as u32) << 16) + ((read_byte(reader, line, file).await? as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1111_0000..=0b1111_0111 => 
            match char::from_u32(((current as u32) << 24) + ((read_byte(reader, line, file).await? as u32) << 16) + ((read_byte(reader, line, file).await? as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        _ => Error::InvalidStructure.into()
    }
}
pub trait DataReader: Sized {
    fn read(reader: &mut (impl AsyncRead + Unpin)) -> impl Future<Output = Result<Self>>;
}

pub trait PacketReader: Sized {
    fn read(
        reader: &mut (impl AsyncRead + Unpin),
        length: i32,
        packet_id: i32,
    ) -> impl Future<Output = Result<Self>>;
}

pub trait DataWriter {
    fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> impl Future<Output = Result<()>>;
}

pub trait ListDataReader: Sized {
    fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> impl Future<Output = Result<Self>> where Self: Sized;
}
pub trait Packet: Sized {}
