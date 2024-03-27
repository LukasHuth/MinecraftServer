use crate::{traits::NbtRead, NbtData, error::{NbtResult, NbtError}, NbtValue};

impl NbtRead for NbtData {
    fn read_i8(reader: &mut NbtData) -> NbtResult<i8> {
        todo!()
    }

    fn read_i16(reader: &mut NbtData) -> NbtResult<i16> {
        todo!()
    }

    fn read_i32(reader: &mut NbtData) -> NbtResult<i32> {
        todo!()
    }

    fn read_i64(reader: &mut NbtData) -> NbtResult<i64> {
        todo!()
    }

    fn read_f32(reader: &mut NbtData) -> NbtResult<f32> {
        todo!()
    }

    fn read_f64(reader: &mut NbtData) -> NbtResult<f64> {
        todo!()
    }

    fn read_i8_array(reader: &mut NbtData) -> NbtResult<Vec<i8>> {
        todo!()
    }

    fn read_nbt_string(reader: &mut NbtData) -> NbtResult<String> {
        todo!()
    }

    fn read_list(reader: &mut NbtData) -> NbtResult<Vec<NbtValue>> {
        todo!()
    }

    fn read_compound(reader: &mut NbtData) -> NbtResult<Vec<(String, NbtValue)>> {
        let mut compound = Vec::new();
        loop {
            let tag_id = reader.read_u8()?;
            if tag_id == 0 { break; }
            let name_length = reader.read_var_i32()?;
            let name = reader.read_string(name_length as usize)?;
            // https://github.com/shenjackyuanjie/nbt-rust/blob/main/shen-nbt5/src/reader.rs#L42
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_u8()? as i8),
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),
                5 => todo!(),
                6 => todo!(),
                7 => todo!(),
                8 => todo!(),
                9 => todo!(),
                10 => todo!(),
                11 => todo!(),
                12 => todo!(),
                _ => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.push((name, value));
        }
        Ok(compound)
    }

    fn read_i32_array(reader: &mut NbtData) -> NbtResult<Vec<i32>> {
        todo!()
    }

    fn read_i64_array(reader: &mut NbtData) -> NbtResult<Vec<i64>> {
        todo!()
    }
}
