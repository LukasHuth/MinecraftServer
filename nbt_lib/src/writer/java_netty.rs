use crate::{version::JavaNetty, traits::NbtWrite};

impl NbtWrite for JavaNetty {
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]) {
        todo!()
    }

    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]) {
        todo!()
    }

    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]) {
        todo!()
    }

    fn write_nbt_string(writer: &mut Vec<u8>, data: &str) {
        todo!()
    }

    fn write_list(writer: &mut Vec<u8>, data: &[crate::NbtValue]) -> crate::error::NbtResult<()> {
        todo!()
    }

    fn write_compound(writer: &mut Vec<u8>, name: Option<&String>, data: &[(String, crate::NbtValue)]) -> crate::error::NbtResult<()> {
        todo!()
    }

    fn write_to(value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        todo!()
    }

    fn write_to_with_name(name: &str, value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        todo!()
    }
}
