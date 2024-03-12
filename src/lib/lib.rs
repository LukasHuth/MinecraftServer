#![feature(str_from_utf16_endian)]

use datatypes::necesary::Necesary;
use crate::datatypes::{Identifier, VarInt};

#[allow(dead_code)]
pub mod datatypes;
#[cfg(test)]
mod test;

pub fn test() {
    let test: [u8;5] = [0x80, 0x80, 0x80, 0x80, 0x08];
    let mut vl = VarInt::new(0);
    let mut ti = test.iter();
    vl.read(&mut ti, None);
    println!("{}|2147483647", vl.get_value());
    let mut arr = vec![];
    vl.write(&mut arr);
    println!("{:?}", arr);
    let ident = Identifier::new(String::from("hallo"));
    println!("{}", ident.get_value());
}
