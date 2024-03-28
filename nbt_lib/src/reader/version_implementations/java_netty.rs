use crate::{traits::NbtRead, version::{JavaNetty, Java}, NbtValue, error::NbtError};

impl NbtRead for JavaNetty {
    fn read_i8_array(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<Vec<i8>> {
        Java::read_i8_array(reader)
    }

    fn read_i32_array(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<Vec<i32>> {
        Java::read_i32_array(reader)
    }

    fn read_i64_array(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<Vec<i64>> {
        Java::read_i64_array(reader)
    }

    fn read_nbt_string(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<String> {
        Java::read_nbt_string(reader)
    }

    fn read_list(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<Vec<crate::NbtValue>> {
        Java::read_list(reader)
    }

    fn read_compound(reader: &mut crate::reader::NbtReader) -> crate::error::NbtResult<Vec<(String, crate::NbtValue)>> {
        Java::read_compound(reader)
    }

    fn from_reader(mut reader: crate::reader::NbtReader) -> crate::error::NbtResult<crate::NbtValue> {
        match reader.read_u8()? {
            10 => Ok(NbtValue::Compound(None, Java::read_compound(&mut reader)?)),
            x => Err(NbtError::WrongRootType(x)),
        }
    }
}
