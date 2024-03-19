use std::io::{Read, Write, BufRead};
use crate::errors::{Result, Error};

pub(crate) fn read_byte(reader: &mut impl Read) -> Result<u8> {
    let mut data = [0u8; 1];
    match reader.read_exact(&mut data) {
        Ok(_) => Ok(data[0]),
        Err(_) => Error::NotEnoughtBytes.into(),
    }
}
pub(crate) fn peek(reader: &mut dyn BufRead) -> Result<Vec<u8>> {
    match reader.fill_buf() {
        Ok(v) => Ok(v.to_vec()),
        Err(_) => Error::NotEnoughtBytes.into(),
    }
}
pub(crate) fn consume_utf16be_char(reader: &mut impl Read) -> Result<()> {
    if read_byte(reader)? & 0xFC == 0xD8 {
        read_byte(reader)?;
    }
    Ok(())
}
pub(crate) fn consume_n_bytes(reader: &mut impl Read, n: u64) -> Result<()> {
    for _ in 0..n {
        read_byte(reader)?;
    }
    Ok(())
}
pub(crate) fn write_bytes(writer: &mut impl Write, bytes: &[u8]) -> Result<()> {
    match writer.write_all(bytes) {
        Ok(_) => Ok(()),
        Err(_) => Error::FailedToWrite.into(),
    }
}
pub(crate) fn read_utf8_char(reader: &mut impl Read) -> Result<char> {
    let current = read_byte(reader)? as u8;
    match current {
        0b0000_0000..=0b0111_1111 => 
            match char::from_u32(current as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1100_0000..=0b1101_1111 => 
            match char::from_u32(((current as u32) << 8) + read_byte(reader)? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1110_0000..=0b1110_1111 => 
            match char::from_u32(((current as u32) << 16) + ((read_byte(reader)? as u32) << 8) + read_byte(reader)? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1111_0000..=0b1111_0111 => 
            match char::from_u32(((current as u32) << 24) + ((read_byte(reader)? as u32) << 16) + ((read_byte(reader)? as u32) << 8) + read_byte(reader)? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        _ => Error::InvalidStructure.into()
    }
}
pub trait DataReader {
    fn read(reader: &mut impl Read) -> Result<Self> where Self: Sized;
}
pub trait DataWriter {
    fn write(&self, writer: &mut impl Write) -> Result<()>;
}
pub trait ListDataReader {
    fn read_list(reader: &mut impl Read, length: usize) -> Result<Self> where Self: Sized;
}
