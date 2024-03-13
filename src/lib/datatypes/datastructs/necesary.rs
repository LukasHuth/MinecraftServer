use std::{io::{BufReader, Read}, net::TcpStream};

use super::*;
pub trait Necesary {
    type Value;
    fn new(value: Self::Value) -> Self;
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self;
    fn write(&self, arr: &mut Vec<u8>);
    fn get_value(&self) -> Self::Value;
}
pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BITS: u8 = 0x80;
impl Necesary for Int {
    type Value = i32;

    fn new(value: Self::Value) -> Self {
        Self(value)
    }

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

    fn get_value(&self) -> Self::Value {
        self.0
    }
}
impl Necesary for VarInt {
    fn new(value: Self::Value) -> Self {
        let mut i = 1;
        let mut d = value;
        while d > 0 {
            d >>= 7;
            if d > 0 { i += 1; }
        }
        assert!(i <= 5, "VarInt is too big");
        Self(i,value)
    }
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut v0 = 0;
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
        v0 = position / 7 + 1;
        Self(v0, v1)
    }
    fn write(&self, arr: &mut Vec<u8>) {
        println!("writing!");
        let mut value = self.1;
        println!("value: {value}");
        for i in 0..self.0 {
            let mut v = value & SEGMENT_BITS as i32;
            println!("v: {v}");
            value >>= 7;
            if i != self.0-1 {
                v = v | CONTINUE_BITS as i32;
            }
            arr.push(v as u8);
        }
    }
    type Value = i32;
    fn get_value(&self) -> Self::Value {
        return self.1;
    }
}
impl Necesary for VarLong {
    fn new(value: Self::Value) -> Self {
        let mut i = 1;
        let mut d = value;
        while d > 0 {
            d >>= 7;
            if d > 0 { i += 1; }
        }
        assert!(i <= 5, "VarInt is too big");
        Self(i,value)
    }
    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut v0 = 0;
        let mut v1 = 0;
        let mut position = 0;
        loop {
            let current_byte = read_byte!(reader);
            v1 |= ((current_byte & SEGMENT_BITS) as i64) << position;
            println!("Hello!");
            println!("{}|{}|{}", current_byte, CONTINUE_BITS, current_byte&CONTINUE_BITS);
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 64, "VarLong is too big");
        }
        v0 = position / 7 + 1;
        Self(v0, v1)
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut value = self.1;
        for i in 0..self.0 {
            let mut v = value & SEGMENT_BITS as i64;
            value >>= 7;
            if i != self.0-1 {
                v = v | CONTINUE_BITS as i64;
            }
            arr.push(v as u8);
        }
    }
    type Value = i64;
    fn get_value(&self) -> Self::Value {
        return self.1;
    }
}
impl Necesary for Identifier {
    type Value = std::string::String;

    fn new(value: Self::Value) -> Self {
        Self(value)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Self {
    // fn read<'a, I>(&mut self, _arr: &mut I, length: Option<u64>) where I: Iterator<Item = &'a u8> {
        let _length = length.expect("No length specified");
        todo!();
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }

    fn get_value(&self) -> Self::Value {
        return self.0.clone();
    }
}
impl Necesary for Position {
    type Value = (i32, i32, i16);

    fn new(value: Self::Value) -> Self {
        let x = I26::new(value.0).expect("x value is larger than 26 bytes");
        let z = I26::new(value.1).expect("z value is larger than 26 bytes");
        let y = I12::new(value.2).expect("y value is larger than 12 bytes");
        Self(x, z, y)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
    // fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        const X_LENGTH: u8 = 26;
        const Y_LENGTH: u8 = 12;
        const Z_LENGTH: u8 = 26;
        let mut long: u64 = 0;
        for _ in 0..4 {
            long <<= 8;
            long |= read_byte!(reader) as u64;
        }
        // <x><z><y>
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

    fn get_value(&self) -> Self::Value {
        todo!()
    }
}
impl Necesary for Long {
    type Value = i64;
    fn new(value: Self::Value) -> Self {
        Self(value)
    }

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

    fn get_value(&self) -> Self::Value {
        self.0
    }
}
impl Necesary for Byte {
    type Value = i8;
    fn new(value: Self::Value) -> Self {
        Self(value)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        Self(read_byte!(reader) as i8)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(self.0 as u8);
    }

    fn get_value(&self) -> Self::Value {
        self.0
    }
}
impl Necesary for BitSet {
    type Value = Vec<Long>;

    fn new(value: Self::Value) -> Self {
        Self(VarInt::from(value.len() as i32), value)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let length = VarInt::read(reader, None);
        let result: Vec<Long> = (0..length.get_value()).map(|_| Long::read(reader, None)).collect();
        Self(length, result)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.0.write(arr);
        self.1.iter().for_each(|l| l.write(arr));
    }

    fn get_value(&self) -> Self::Value {
        self.1.clone()
    }
}
impl Necesary for FixedBitSet {
    type Value = (u64, Vec<Byte>);

    fn new(value: Self::Value) -> Self {
        Self(value.0, value.1)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Self {
        let length = length.expect("Expected a given length");
        let result: Vec<Byte> = (0..length).map(|_| Byte::read(reader, None)).collect();
        Self(length, result)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.1.iter().for_each(|l| l.write(arr));
    }

    fn get_value(&self) -> Self::Value {
        (self.0, self.1.clone())
    }
}
impl Necesary for String {
    type Value = std::string::String;

    fn new(value: Self::Value) -> Self {
        Self(VarInt::from(value.len() as i32),  value)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let length = VarInt::read(reader, None);
        println!("String length: {}", length.get_value());
        let mut data = vec![];
        for _ in 0..length.get_value() {
            data.push(read_byte!(reader));
        }
        let st = std::string::String::from_utf8(data).expect("Could not convert data to utf8 string");
        Self(length, st)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.0.write(arr);
        for byte in self.1.as_bytes() {
            arr.push(0);
            arr.push(*byte);
        }
    }

    fn get_value(&self) -> Self::Value {
        self.1.clone()
    }
}
impl Necesary for UnsignedShort {
    type Value = u16;

    fn new(value: Self::Value) -> Self {
        Self(value)
    }

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

    fn get_value(&self) -> Self::Value {
        self.0
    }
}
impl Necesary for UUID {
    type Value = u128;

    fn new(value: Self::Value) -> Self {
        Self((value >> 64) as u64, value as u64)
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, _: Option<u64>) -> Self {
        let mut data = 0;
        for _ in 0..16 {
            data <<= 8;
            data |= read_byte!(reader) as u128;
        }
        Self::new(data)
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut offset = 56;
        for _ in 0..8 {
            arr.push(((self.0 >> offset) & 0xFF) as u8);
        }
        offset = 56;
        for _ in 0..8 {
            arr.push(((self.1 >> offset) & 0xFF) as u8);
        }
    }

    fn get_value(&self) -> Self::Value {
        ((self.0 as u128) << 64) + self.1 as u128
    }
}
impl Necesary for ByteArray {
    type Value = Vec<u8>;

    fn new(value: Self::Value) -> Self {
        Self(value.iter().map(|b|Byte::new(*b as i8)).collect())
    }

    fn read(reader: &mut BufReader<&mut TcpStream>, length: Option<u64>) -> Self {
        let length = length.expect("ByteArray needs an specified length to read");
        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data);
        Self::new(data)
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.append(&mut self.0.iter().map(|b|b.0 as u8).collect());
    }

    fn get_value(&self) -> Self::Value {
        self.0.iter().map(|b|b.0 as u8).collect()
    }
}
