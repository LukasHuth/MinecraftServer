use crate::{traits::NbtWrite, version::Java, NbtValue, error::{NbtResult, NbtError}};

impl NbtWrite for Java {
    #[cfg_attr(feature = "inline_read", inline)]
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]) {
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        writer.extend_from_slice(data.iter().map(|x| *x as u8).collect::<Vec<u8>>().as_slice());
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]) {
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        writer.extend_from_slice(&data.iter().map(|x| x.to_be_bytes()).collect::<Vec<[u8; 4]>>().concat());
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]) {
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        writer.extend_from_slice(&data.iter().map(|x| x.to_be_bytes()).collect::<Vec<[u8; 8]>>().concat());
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str) {
        writer.extend_from_slice(&(data.len() as u16).to_be_bytes());
        writer.extend_from_slice(data.as_bytes());
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_compound(writer: &mut Vec<u8>, name: Option<&String>, data: Vec<(&String, &NbtValue)>) -> NbtResult<()> {
        Self::write_nbt_string(writer, name.unwrap_or(&"".to_string()));
        for (key, value) in data {
            writer.push(value.tag() as u8);
            Self::write_nbt_string(writer, key);
            match value {
                NbtValue::Byte(v) => writer.push(*v as u8),
                NbtValue::Short(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Int(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Long(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Float(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Double(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::ByteArray(v) => Self::write_i8_array(writer, v),
                NbtValue::IntArray(v) => Self::write_i32_array(writer, v),
                NbtValue::LongArray(v) => Self::write_i64_array(writer, v),
                NbtValue::String(v) => Self::write_nbt_string(writer, v),
                NbtValue::List(v) => Self::write_list(writer, v)?,
                NbtValue::Compound(name, v) => {
                    Self::write_compound(writer,
                    name.as_ref(), v.iter().collect()
                    )?
                }
            }
        }
        writer.push(0);
        Ok(())
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_to(value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        match value {
            NbtValue::Compound(name, data) => {
                buff.push(value.tag() as u8);
                Self::write_compound(buff, name.as_ref(), data.iter().collect())?
            }
            v => return Err(NbtError::WrongRootType(v.tag() as u8)),
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_to_with_name(name: &str, value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        buff.push(value.tag() as u8);
        Self::write_nbt_string(buff, name);
        Self::write_to(value, buff)
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_list(writer: &mut Vec<u8>, data: &[NbtValue]) -> NbtResult<()> {
        if data.is_empty() {
            writer.extend_from_slice(&0i8.to_be_bytes());
            return Ok(());
        }
        let tag = data.first().unwrap().tag();
        if !data.iter().all(|x| x.tag() == tag) {
            return Err(NbtError::ListTypeNotSame(data.iter().map(|x| x.tag()).collect()));
        }
        writer.push(tag as u8);
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());

        for d in data {
            match d {
                NbtValue::Byte(v) => writer.push(*v as u8),
                NbtValue::Short(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Int(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Long(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Float(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::Double(v) => writer.extend_from_slice(&v.to_be_bytes()),
                NbtValue::ByteArray(v) => Self::write_i8_array(writer, v),
                NbtValue::IntArray(v) => Self::write_i32_array(writer, v),
                NbtValue::LongArray(v) => Self::write_i64_array(writer, v),
                NbtValue::String(v) => Self::write_nbt_string(writer, v),
                NbtValue::List(v) => Self::write_list(writer, v)?,
                NbtValue::Compound(name, v) => {
                    Self::write_compound(writer, name.as_ref(), v.iter().collect())?
                }
            }
        }
        Ok(())
    }

    #[cfg_attr(feature = "inline_read", inline)]
    fn write_text_component(writer: &mut Vec<u8>, value: &NbtValue) -> NbtResult<()> {
        match value {
            NbtValue::String(str) => Ok(Java::write_nbt_string(writer, str)),
            NbtValue::Compound(_, _) => Java::write_to(value, writer),
            v => Err(NbtError::WrongRootType(v.tag() as u8)),
        }
    }
}
