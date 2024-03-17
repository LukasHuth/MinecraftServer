use std::io::{Cursor, Read};

use byte_tag::NetNBTByteTag;
use end_tag::NBTEndTag;
use string_tag::NetNBTStringTag;
use utils::DataReader as _;

mod utility;

mod tag_name;
mod string_tag;
mod end_tag;
mod byte_tag;

trait NBTTest {
    fn is_this(reader: &mut impl Read) -> bool;
}

enum NetNBTTag {
    String(NetNBTStringTag),
    Byte(NetNBTByteTag),
    End(NBTEndTag),
}

#[test]
pub fn test() {
    let data = [ 1, 25 ];
    let mut cursor = Cursor::new(data.to_vec());
    match NetNBTByteTag::read(&mut cursor) {
        Ok(v) => println!("{}", v.data),
        Err(_) => (),
    }
    // let test: NetNBTByteTag = bincode::deserialize_from(&mut cursor).unwrap();
    // let test: NBTEndTag = bincode::deserialize_from(&mut cursor).unwrap();
}
