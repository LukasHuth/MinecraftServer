#![feature(str_from_utf16_endian)]
extern crate serde;
macro_rules! read_byte {
    ($var:expr) => {
        $var.bytes().next().transpose().expect("Error reading a byte").expect("Expected byte to read")
    };
}

#[allow(dead_code)]
pub mod datatypes;

#[cfg(test)]
mod test;

pub fn test() {
}
