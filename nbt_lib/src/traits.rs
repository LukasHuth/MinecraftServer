use crate::{NbtValue, error::NbtResult, reader::NbtReader};

pub trait NbtWrite {
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]);
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]);
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]);
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str);
    fn write_compound(writer: &mut Vec<u8>, name: Option<String>, data: &[(String, NbtValue)]) -> NbtResult<NbtValue>;
    fn write_to(value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;
    fn write_to_with_name(name: &str, value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;
    fn to_bytes(value: &NbtValue) -> NbtResult<Vec<u8>> {
        let mut buff = Vec::new();
        Self::write_to(value, &mut buff)?;
        Ok(buff)
    }
}

pub trait NbtRead {
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>>;
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>>;
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>>;
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String>;
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>>;
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>>;
    fn from_reader(reader: NbtReader) -> NbtResult<NbtValue>;
}
