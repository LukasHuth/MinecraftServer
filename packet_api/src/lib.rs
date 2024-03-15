#![feature(str_from_utf16_endian)]
#![feature(buf_read_has_data_left)]
extern crate derives;
extern crate uuid;
trait TestNeccessaryTrait {
    type Value;
    fn new(value: Self::Value) -> Self;
    fn get_value(&self) -> &Self::Value;
}

use datatypes::datastructs::{VarInt, necesary::Necesary};
extern crate serde;
macro_rules! read_byte {
    ($var:expr) => {
        $var.bytes().next().transpose().expect("Error reading a byte").expect("Expected byte to read")
    };
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
