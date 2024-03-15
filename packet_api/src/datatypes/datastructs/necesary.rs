use std::{io::{BufReader, Read}, net::TcpStream};

use super::*;
pub trait Necesary {
    type Value;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self;
    fn write(&self, arr: &mut Vec<u8>);
}
pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BITS: u8 = 0x80;
impl Necesary for Int {
    type Value = i32;

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut data = [0;4];
        reader.read_exact(&mut data).expect("Expected bytes to read");
        let mut value = 0;
        for v in data {
            value <<= 8;
            value += v as u32;
        }
        Self(value as i32)
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for VarInt {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut v1 = 0;
        v1 = 0;
        let mut position = 0;
        loop {
            // let current_byte = reader.bytes().next().transpose().unwrap_or(Some(0)).unwrap_or(0);
            let current_byte = read_byte!(reader);
            // let current_byte = read_byte!(reader_clone).unwrap_or(Some(0)).unwrap_or(0);
            v1 |= ((current_byte & SEGMENT_BITS) as i32) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 32, "VarInt is too big");
        }
        Self::new(v1)
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
    type Value = i32;
}
impl Necesary for VarLong {
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut v0 = 0;
        let mut v1 = 0;
        let mut position = 0;
        loop {
            let current_byte = read_byte!(reader);
            v1 |= ((current_byte & SEGMENT_BITS) as i64) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 64, "VarLong is too big");
        }
        v0 = position / 7 + 1;
        Self(v1)
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
    type Value = i64;
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
    type Value = std::string::String;

    fn read(reader: &mut BufReader<&mut TcpStream>, _length: Option<u64>) -> Self {
        let value = String::read(reader, None);
        Self(value.get_value().clone())
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for Position {
    type Value = (i32, i32, i16);

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        const X_LENGTH: u8 = 26;
        const Y_LENGTH: u8 = 12;
        const Z_LENGTH: u8 = 26;
        let mut long: u64 = 0;
        for _ in 0..4 {
            long <<= 8;
            long |= read_byte!(reader) as u64;
        }
        let x = long >> (Y_LENGTH + Z_LENGTH);
        let y = long << (Z_LENGTH+X_LENGTH) >> (Z_LENGTH+X_LENGTH);
        let z = long << X_LENGTH >> (X_LENGTH+Y_LENGTH);
        let x = I26::new(x as i32).expect("Couldn't build x variable");
        let y = I12::new(y as i16).expect("Couldn't build y variable");
        let z = I26::new(z as i32).expect("Couldn't build z variable");
        Self(x, z, y)
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }
}
impl Necesary for Long {
    type Value = i64;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut long = 0;
        for _ in 0..8 {
            long >>= 8;
            long += read_byte!(reader) as u64;
        }
        Self(long as i64)
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
    type Value = i8;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        Self(read_byte!(reader) as i8)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(self.0 as u8);
    }
}
impl Necesary for BitSet {
    type Value = Vec<Long>;

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let length = VarInt::read(reader, None);
        let result: Vec<Long> = (0..length.get_value()).map(|_| Long::read(reader, None)).collect();
        Self(length, result)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.0.write(arr);
        self.1.iter().for_each(|l| l.write(arr));
    }
}
impl Necesary for FixedBitSet {
    type Value = (u64, Vec<Byte>);
    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Self {
        let length = length.expect("Expected a given length");
        let result: Vec<Byte> = (0..length).map(|_| Byte::read(reader, None)).collect();
        Self(length, result)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.1.iter().for_each(|l| l.write(arr));
    }
}
impl Necesary for String {
    type Value = std::string::String;

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let length = VarInt::read(reader, None);
        let mut data = vec![];
        for _ in 0..length.get_value() {
            let byte = read_byte!(reader);
            data.push(byte);
            let size = match byte {
                0b0000_0000..=0b0111_1111 => 1,
                0b1100_0000..=0b1101_1111 => 2,
                0b1110_0000..=0b1110_1111 => 3,
                0b1111_0000..=0b1111_0111 => 4,
                _ => panic!("Error in the UTF8 coding"),
            };
            for _ in 1..size {
                data.push(read_byte!(reader));
            }
        }
        let st = std::string::String::from_utf8(data).expect("Could not convert data to utf8 string");
        Self { data: st }
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
    type Value = u16;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut v = 0;
        for _ in 0..2 {
            v <<= 8;
            v += read_byte!(reader) as u16;
        }
        Self(v)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push((self.0 >> 8) as u8);
        arr.push((self.0 & 0xFF) as u8);
    }
}
impl Necesary for UUID {
    type Value = u128;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut data = 0;
        for _ in 0..16 {
            data <<= 8;
            data |= read_byte!(reader) as u128;
        }
        Self::new(data)
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
    type Value = Vec<u8>;
    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Self {
        let length = length.expect("ByteArray needs an specified length to read");
        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data).expect("Failed to read provided amount of bytes");
        Self::new(data.iter().map(|b| Byte::new(*b as i8)).collect())
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.append(&mut self.0.iter().map(|b|b.0 as u8).collect());
    }
}
impl Necesary for Boolean {
    type Value = bool;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let data = read_byte!(reader);
        Self::new(data != 0x00)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(if self.0 { 0x01 } else { 0x00 })
    }
}
