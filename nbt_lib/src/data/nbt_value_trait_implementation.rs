use crate::{traits::NbtRead, NbtData, error::{NbtResult, NbtError}, NbtValue};

impl NbtRead for NbtValue {
    fn read_i8_array(reader: &mut NbtData) -> NbtResult<Vec<i8>> {
        let len = reader.read_be_u32()? as i32;
        let data = reader.read_i8_array(len as usize)?;
        Ok(data)
    }

    fn read_nbt_string(reader: &mut NbtData) -> NbtResult<String> {
        let len = reader.read_le_u16()?;
        println!("string_len: {len}");
        let data = reader.read_string(len as usize)?;
        Ok(data)
    }

    fn read_list(reader: &mut NbtData) -> NbtResult<Vec<NbtValue>> {
        let type_id = reader.read_u8()?;
        let len = reader.read_be_u32()?;
        if len <= 0 { return Ok(vec![]); }
        let mut elements = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let value = match type_id {
                1 => NbtValue::Byte(reader.read_u8()? as i8),
                2 => NbtValue::Short(reader.read_be_u16()? as i16),
                3 => NbtValue::Int(reader.read_be_u32()? as i32),
                4 => NbtValue::Long(reader.read_be_u64()? as i64),
                5 => NbtValue::Float(reader.read_be_f32()?),
                6 => NbtValue::Double(reader.read_be_f64()?),
                7 => NbtValue::ByteArray(Self::read_i8_array(reader)?),
                8 => NbtValue::String(Self::read_nbt_string(reader)?),
                9 => NbtValue::List(Self::read_list(reader)?),
                10 => NbtValue::Compound(Self::read_compound(reader, true)?),
                11 => NbtValue::IntArray(Self::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Self::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(type_id)),
            };
            elements.push(value);
        }
        Ok(elements)
    }

    fn read_compound(reader: &mut NbtData, with_name: bool) -> NbtResult<(Option<String>, Vec<(String, NbtValue)>)> {
        let mut compound = Vec::new();
        let name = if with_name { Some(Self::read_nbt_string(reader)?) } else { None };
        loop {
            println!("{}", reader);
            let tag_id = reader.read_u8()?;
            println!("{}", reader);
            println!("tag_id: {tag_id}");
            if tag_id == 0 { break; }
            let name_length = reader.read_be_u16()?;
            println!("{}", reader);
            let name = reader.read_string(name_length as usize)?;
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_u8()? as i8),
                2 => NbtValue::Short(reader.read_be_u16()? as i16),
                3 => NbtValue::Int(reader.read_be_u32()? as i32),
                4 => NbtValue::Long(reader.read_be_u64()? as i64),
                5 => NbtValue::Float(reader.read_be_f32()?),
                6 => NbtValue::Double(reader.read_be_f64()?),
                7 => NbtValue::ByteArray(Self::read_i8_array(reader)?),
                8 => NbtValue::String(Self::read_nbt_string(reader)?),
                9 => NbtValue::List(Self::read_list(reader)?),
                10 => NbtValue::Compound(Self::read_compound(reader, true)?),
                11 => NbtValue::IntArray(Self::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Self::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.push((name, value));
        }
        Ok((name, compound))
    }

    fn read_i32_array(reader: &mut NbtData) -> NbtResult<Vec<i32>> {
        let len = reader.read_be_u32()? as i32;
        let mut data = Vec::new();
        for _ in 0..len {
            data.push(reader.read_be_u32()? as i32);
        }
        Ok(data)
    }

    fn read_i64_array(reader: &mut NbtData) -> NbtResult<Vec<i64>> {
        let len = reader.read_be_u32()? as i32;
        let mut data = Vec::new();
        for _ in 0..len {
            data.push(reader.read_be_u64()? as i64);
        }
        Ok(data)
    }
}
