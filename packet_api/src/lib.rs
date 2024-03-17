#![feature(str_from_utf16_endian)]
#![feature(buf_read_has_data_left)]
extern crate derives;
extern crate uuid;
trait TestNeccessaryTrait {
    type Value;
    fn new(value: Self::Value) -> Self;
    fn get_value(&self) -> &Self::Value;
}

use std::{io::{BufReader, Read as _}, net::TcpStream};

use datatypes::datastructs::{VarInt, necesary::Necesary};
extern crate serde;
macro_rules! _read_byte {
    ($var:expr) => {
        $var.bytes().next().transpose().expect("Error reading a byte").expect("Expected byte to read")
    };
}
pub enum DatastructError {
    ByteReadingFailed,
    ConvertionError,
    PacketCreationFailed,
    InvalidPacket,
}
fn read_byte(reader: &mut BufReader<&mut TcpStream>) -> Result<u8, DatastructError> {
    Ok(match reader.bytes().next().transpose() {
        Ok(current_byte) => Ok(match current_byte {
            Some(current_byte) => Ok(current_byte),
            None => Err(DatastructError::ByteReadingFailed),
        }?),
        Err(_) => Err(DatastructError::ByteReadingFailed),
    }?)
}

pub mod datatypes;

#[cfg(test)]
mod test;

pub fn test() {
    let vi = VarInt::new(127);
    let mut test = vec![];
    vi.write(&mut test);
    println!("{:?}", test);
    let test = datatypes::datastructs::UnsignedByte::new(0);
    println!("test_data: {}", test.get_value());
}
