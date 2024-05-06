use std::collections::HashMap;

use crate::{traits::NbtRead, version::Java, reader::NbtReader, error::{NbtResult, NbtError}, NbtValue};

impl NbtRead for Java {
    #[cfg_attr(feature = "inline_read", inline)]
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>> {
        let len = reader.read_be_u32()? as usize;
        reader.read_i8_array(len)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>> {
        let len = reader.read_be_u32()? as usize;
        reader.read_be_i32_array(len)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>> {
        let len = reader.read_be_u32()? as usize;
        reader.read_be_i64_array(len)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String> {
        let len = reader.read_be_u16()? as usize;
        reader.read_string(len)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> {
        let type_id = reader.read_u8()?;
        let len = reader.read_be_i32()?;
        let len = if len < 0 { 0 } else { len } as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            let value = match type_id {
                1 => NbtValue::Byte(reader.read_i8()?),
                2 => NbtValue::Short(reader.read_be_i16()?),
                3 => NbtValue::Int(reader.read_be_i32()?),
                4 => NbtValue::Long(reader.read_be_i64()?),
                5 => NbtValue::Float(reader.read_be_f32()?),
                6 => NbtValue::Double(reader.read_be_f64()?),
                7 => NbtValue::ByteArray(Java::read_i8_array(reader)?),
                8 => NbtValue::String(Java::read_nbt_string(reader)?),
                9 => NbtValue::List(Java::read_list(reader)?),
                10 => NbtValue::Compound(None, Java::read_compound(reader)?),
                11 => NbtValue::IntArray(Java::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Java::read_i64_array(reader)?),
                0 => unreachable!("Should be caught before"),
                13..=u8::MAX => return Err(NbtError::UnknownType(type_id)),
            };
            list.push(value);
        }
        Ok(list)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn read_compound(reader: &mut NbtReader) -> NbtResult<HashMap<String, NbtValue>> {
        let mut compound = HashMap::new();
        loop {
            let tag_id = reader.read_u8()?;
            if tag_id == 0 { break }
            let name = Java::read_nbt_string(reader)?;
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_i8()?),
                2 => NbtValue::Short(reader.read_be_i16()?),
                3 => NbtValue::Int(reader.read_be_i32()?),
                4 => NbtValue::Long(reader.read_be_i64()?),
                5 => NbtValue::Float(reader.read_be_f32()?),
                6 => NbtValue::Double(reader.read_be_f64()?),
                7 => NbtValue::ByteArray(Java::read_i8_array(reader)?),
                8 => NbtValue::String(Java::read_nbt_string(reader)?),
                9 => NbtValue::List(Java::read_list(reader)?),
                10 => NbtValue::Compound(None, Java::read_compound(reader)?),
                11 => NbtValue::IntArray(Java::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Java::read_i64_array(reader)?),
                0 => unreachable!("Should be caught before"),
                13..=u8::MAX => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.insert(name, value);
        }
        Ok(compound)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn from_reader(mut reader: NbtReader) -> NbtResult<NbtValue> {
        match reader.read_u8()? {
            10 => {
                let name = Java::read_nbt_string(&mut reader)?;
                Ok(NbtValue::Compound(Some(name), Java::read_compound(&mut reader)?))
            }
            x => Err(NbtError::WrongRootType(x)),
        }
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn from_reader_text_component(mut reader: NbtReader) -> NbtResult<NbtValue> {
        match reader.read_u8()? {
            10 => {
                let name = Java::read_nbt_string(&mut reader)?;
                Ok(NbtValue::Compound(Some(name), Java::read_compound(&mut reader)?))
            }
            8 => {
                Ok(NbtValue::String(Java::read_nbt_string(&mut reader)?))
            }
            x => Err(NbtError::WrongRootType(x)),
        }
    }
}
