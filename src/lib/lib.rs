#![feature(str_from_utf16_endian)]
#![feature(buf_read_has_data_left)]
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
}
