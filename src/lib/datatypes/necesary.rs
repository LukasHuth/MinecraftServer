use super::*;
pub trait Necesary {
    type Value;
    fn new(value: Self::Value) -> Self;
    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8>;
    fn write(&self, arr: &mut Vec<u8>);
    fn get_value(&self) -> Self::Value;
}
const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BITS: u8 = 0x80;
impl Necesary for Int {
    type Value = i32;

    fn new(value: Self::Value) -> Self {
        Self(value)
    }

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        self.0 = 0;
        for _ in 0..4 {
            self.0 <<= 8;
            self.0 |= *arr.next().expect("Expected byte to read") as i32;
        }
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
    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>)
    where I: Iterator<Item = &'a u8>{
        self.1 = 0;
        let mut position = 0;
        loop {
            let current_byte = arr.next().unwrap_or(&0);
            self.1 |= ((current_byte & SEGMENT_BITS) as i32) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 32, "VarInt is too big");
        }
        self.0 = position / 7 + 1;
    }
    fn write(&self, arr: &mut Vec<u8>) {
        let mut value = self.1;
        for i in 0..self.0 {
            let mut v = value & SEGMENT_BITS as i32;
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
    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>)
    where I: Iterator<Item = &'a u8>{
        self.1 = 0;
        let mut position = 0;
        loop {
            let current_byte = arr.next().unwrap_or(&0);
            self.1 |= ((current_byte & SEGMENT_BITS) as i64) << position;
            println!("Hello!");
            println!("{}|{}|{}", current_byte, CONTINUE_BITS, current_byte&CONTINUE_BITS);
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 64, "VarLong is too big");
        }
        self.0 = position / 7 + 1;
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

    fn read<'a, I>(&mut self, _arr: &mut I, length: Option<u64>) where I: Iterator<Item = &'a u8> {
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

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        const X_LENGTH: u8 = 26;
        const Y_LENGTH: u8 = 12;
        const Z_LENGTH: u8 = 26;
        let mut long: u64 = 0;
        for _ in 0..4 {
            long <<= 8;
            long |= *arr.next().expect("Expected an byte to read") as u64;
        }
        // <x><z><y>
        let x = long >> (Y_LENGTH + Z_LENGTH);
        let y = long << (Z_LENGTH+X_LENGTH) >> (Z_LENGTH+X_LENGTH);
        let z = long << X_LENGTH >> (X_LENGTH+Y_LENGTH);
        let x = I26::new(x as i32).expect("Couldn't build x variable");
        let y = I12::new(y as i16).expect("Couldn't build y variable");
        let z = I26::new(z as i32).expect("Couldn't build z variable");
        self.0 = x;
        self.1 = z;
        self.2 = y;
    }

    fn write(&self, _arr: &mut Vec<u8>) {
        todo!()
    }

    fn get_value(&self) -> Self::Value {
        todo!()
    }
}
impl Long {
    pub fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let mut long: u64 = 0;
        for _ in 0..4 {
            long <<= 8;
            long |= *arr.next().expect("Expected another byte") as u64;
        }
        Self(long as i64)
    }
}
impl Necesary for Long {
    type Value = i64;
    fn new(value: Self::Value) -> Self {
        Self(value)
    }

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        self.0 = Self::read(arr).0;
    }

    fn write(&self, arr: &mut Vec<u8>) {
        arr.push(((self.0 >> 48) & 0xFF) as u8);
        arr.push(((self.0 >> 32) & 0xFF) as u8);
        arr.push(((self.0 >> 16) & 0xFF) as u8);
        arr.push(((self.0 >> 00) & 0xFF) as u8);
    }

    fn get_value(&self) -> Self::Value {
        self.0
    }
}
impl Byte {
    pub fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        Self(*arr.next().expect("Expected a byte to read") as i8)
    }
}
impl Necesary for Byte {
    type Value = i8;
    fn new(value: Self::Value) -> Self {
        Self(value)
    }

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        self.0 = Self::read(arr).0;
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
        Self(VarInt::new(value.len() as i32), value)
    }

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        let mut length = VarInt::new(0);
        length.read(arr, None);
        let result: Vec<Long> = (0..length.get_value()).map(|_| Long::read(arr)).collect();
        self.0 = length;
        self.1 = result;
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

    fn read<'a, I>(&mut self, arr: &mut I, length: Option<u64>) where I: Iterator<Item = &'a u8> {
        let length = length.expect("Expected a given length");
        let result: Vec<Byte> = (0..length).map(|_| Byte::read(arr)).collect();
        self.0 = length;
        self.1 = result;
    }

    fn write(&self, arr: &mut Vec<u8>) {
        self.1.iter().for_each(|l| l.write(arr));
    }

    fn get_value(&self) -> Self::Value {
        (self.0, self.1.clone())
    }
}
impl VarInt {
    pub fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let mut s = VarInt::new(0);
        s.read(arr, None);
        s
    }
}
impl Necesary for String {
    type Value = std::string::String;

    fn new(value: Self::Value) -> Self {
        Self(VarInt::new(value.len() as i32),  value)
    }

    fn read<'a, I>(&mut self, arr: &mut I, _: Option<u64>) where I: Iterator<Item = &'a u8> {
        let length = VarInt::read(arr);
        println!("String length: {}", length.get_value());
        let mut data = vec![];
        for _ in 0..length.get_value() {
            data.push(*arr.next().expect("Expected a byte to read"));
            data.push(*arr.next().expect("Expected a byte to read"));
        }
        let data_as_arr: &[u8] = &data;
        let st = std::string::String::from_utf16be(data_as_arr).expect("Could not convert data to utf16 string");
        self.0 = length;
        self.1 = st;
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
