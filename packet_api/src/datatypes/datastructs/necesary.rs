use std::{io::{BufReader, Read}, net::TcpStream};

use crate::{read_byte, DatastructError};

use super::*;
pub trait Necesary {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError>;
    fn write(&self, arr: &mut Vec<u8>);
}
pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BITS: u8 = 0x80;
impl Necesary for Int {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut data = [0;4];
        reader.read_exact(&mut data).expect("Expected bytes to read");
        let mut value = 0;
        for v in data {
            value <<= 8;
            value += v as u32;
        }
        Ok(Box::new(Self(value as i32)))
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for VarInt {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut v1 = 0;
        v1 = 0;
        let mut position = 0;
        loop {
            let current_byte = read_byte(reader)?;
            v1 |= ((current_byte & SEGMENT_BITS) as i32) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 32, "VarInt is too big");
        }
        Ok(Box::new(Self::new(v1)))
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut value = self.data;
        for i in 0..self.get_bytes() {
            let mut v = value & SEGMENT_BITS as i32;
            value >>= 7;
            if i != self.get_bytes()-1 {
                v = v | CONTINUE_BITS as i32;
            }
            arr.push(v as u8);
        }
    }
}
impl Necesary for VarLong {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut v0 = 0;
        let mut v1 = 0;
        let mut position = 0;
        loop {
            let current_byte = read_byte(reader)?;
            v1 |= ((current_byte & SEGMENT_BITS) as i64) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 64, "VarLong is too big");
        }
        v0 = position / 7 + 1;
        Ok(Box::new(Self(v1)))
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut value = self.0;
        for i in 0..self.get_bytes() {
            let mut v = value & SEGMENT_BITS as i64;
            value >>= 7;
            if i != self.get_bytes()-1 {
                v = v | CONTINUE_BITS as i64;
            }
            arr.push(v as u8);
        }
    }
}
impl VarLong {
    pub fn get_bytes(&self) -> u8 {
        let mut value = self.0 as u128;
        let mut res = 1;
        while value > 0x7F {
            value >>= 7;
            res += 1;
        }
        res
    }
}
impl Necesary for Identifier {
    fn read(reader: &mut BufReader<&mut TcpStream>, _length: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let value = String::read(reader, None)?;
        Ok(Box::new(Self(value.get_value().clone())))
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for Position {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        const X_LENGTH: u8 = 26;
        const Y_LENGTH: u8 = 12;
        const Z_LENGTH: u8 = 26;
        let mut long: u64 = 0;
        for _ in 0..4 {
            long <<= 8;
            long |= read_byte(reader)? as u64;
        }
        let x = long >> (Y_LENGTH + Z_LENGTH);
        let y = long << (Z_LENGTH+X_LENGTH) >> (Z_LENGTH+X_LENGTH);
        let z = long << X_LENGTH >> (X_LENGTH+Y_LENGTH);
        let x = match I26::new(x as i32) {
            Some(v) => Ok(v),
            None => Err(DatastructError::ConvertionError),
        }?;
        let y = match I12::new(y as i16) {
            Some(v) => Ok(v),
            None => Err(DatastructError::ConvertionError),
        }?;
        let z = match I26::new(z as i32) {
            Some(v) => Ok(v),
            None => Err(DatastructError::ConvertionError),
        }?;
        Ok(Box::new(Self(x, z, y)))
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for Long {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut long = 0;
        for _ in 0..8 {
            long >>= 8;
            long += read_byte(reader)? as u64;
        }
        Ok(Box::new(Self(long as i64)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        let mut offset = 56;
        for _ in 0..8 {
            arr.push(((self.0 >> offset) & 0xFF) as u8);
            offset-=8;
        }
    }
}
impl Necesary for Byte {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        Ok(Box::new(Self(read_byte(reader)? as i8)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(self.0 as u8);
    }
}
impl Necesary for BitSet {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let length = VarInt::read(reader, None)?;
        let results = (0..length.get_value()).map(|_| Long::read(reader, None)).collect::<Vec<_>>();
        let mut result = Vec::new();
        for res in results {
            result.push(*res?);
        }
        Ok(Box::new(Self(*length, result)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.0.write(arr);
        self.1.iter().for_each(|l| l.write(arr));
    }
}
impl Necesary for FixedBitSet {
    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let length = length.expect("Expected a given length");
        let results: Vec<Result<Box<Byte>, DatastructError>> = (0..length).map(|_| Byte::read(reader, None)).collect();
        let mut result = Vec::new();
        for res in results {
            result.push(*res?);
        }
        Ok(Box::new(Self(length, result)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.1.iter().for_each(|l| l.write(arr));
    }
}
impl Necesary for String {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let length = VarInt::read(reader, None)?;
        let mut data = vec![];
        for _ in 0..length.get_value() {
            let byte = read_byte(reader)?;
            data.push(byte);
            let size = match byte {
                0b0000_0000..=0b0111_1111 => 1,
                0b1100_0000..=0b1101_1111 => 2,
                0b1110_0000..=0b1110_1111 => 3,
                0b1111_0000..=0b1111_0111 => 4,
                _ => panic!("Error in the UTF8 coding"),
            };
            for _ in 1..size {
                data.push(read_byte(reader)?);
            }
        }
        let st = std::string::String::from_utf8(data).expect("Could not convert data to utf8 string");
        Ok(Box::new(Self { data: st }))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        let length = VarInt::new(self.data.len() as i32);
        length.write(arr);
        for byte in self.data.as_bytes() {
            // arr.push(0);
            arr.push(*byte);
        }
    }
}
impl Necesary for UnsignedShort {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut v = 0;
        for _ in 0..2 {
            v <<= 8;
            v += read_byte(reader)? as u16;
        }
        Ok(Box::new(Self(v)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push((self.0 >> 8) as u8);
        arr.push((self.0 & 0xFF) as u8);
    }
}
impl Necesary for UUID {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let mut data = 0;
        for _ in 0..16 {
            data <<= 8;
            data |= read_byte(reader)? as u128;
        }
        Ok(Box::new(Self::new(data)))
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut offset = 56+64;
        for _ in 0..16 {
            arr.push(((self.0 >> offset) & 0xFF) as u8);
            offset -= 8;
        }
    }
}
impl Necesary for ByteArray {
    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let length = length.expect("ByteArray needs an specified length to read");
        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data).expect("Failed to read provided amount of bytes");
        Ok(Box::new(Self::new(data.iter().map(|b| Byte::new(*b as i8)).collect())))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.append(&mut self.0.iter().map(|b|b.0 as u8).collect());
    }
}
impl Necesary for Boolean {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Result<Box<Self>, DatastructError> {
        let data = read_byte(reader)?;
        Ok(Box::new(Self::new(data != 0x00)))
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(if self.0 { 0x01 } else { 0x00 })
    }
}
