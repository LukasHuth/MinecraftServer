use crate::{version::{JavaNetty, Java}, traits::NbtWrite, NbtValue, error::{NbtError, NbtResult}};

impl NbtWrite for JavaNetty {
    #[inline]
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]) {
        Java::write_i8_array(writer, data)
    }

    #[inline]
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]) {
        Java::write_i32_array(writer, data)
    }

    #[inline]
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]) {
        Java::write_i64_array(writer, data)
    }

    #[inline]
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str) {
        Java::write_nbt_string(writer, data)
    }

    #[inline]
    fn write_list(writer: &mut Vec<u8>, data: &[crate::NbtValue]) -> crate::error::NbtResult<()> {
        Java::write_list(writer, data)
    }

    #[inline]
    fn write_compound(writer: &mut Vec<u8>, name: Option<&String>, data: &[(String, crate::NbtValue)]) -> crate::error::NbtResult<()> {
        Java::write_compound(writer, name, data)
    }

    #[inline]
    fn write_to(value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        match value {
            NbtValue::Compound(_, data) => {
                buff.push(value.tag());
                for (key, value) in data {
                    buff.push(value.tag());
                    Self::write_nbt_string(buff, key);
                    match value {
                        NbtValue::Byte(v) => buff.push(*v as u8),
                        NbtValue::Short(v) => buff.extend_from_slice(&v.to_be_bytes()),
                        NbtValue::Int(v) => buff.extend_from_slice(&v.to_be_bytes()),
                        NbtValue::Long(v) => buff.extend_from_slice(&v.to_be_bytes()),
                        NbtValue::Float(v) => buff.extend_from_slice(&v.to_be_bytes()),
                        NbtValue::Double(v) => buff.extend_from_slice(&v.to_be_bytes()),
                        NbtValue::ByteArray(v) => Self::write_i8_array(buff, v),
                        NbtValue::IntArray(v) => Self::write_i32_array(buff, v),
                        NbtValue::LongArray(v) => Self::write_i64_array(buff, v),
                        NbtValue::String(v) => Self::write_nbt_string(buff, v),
                        NbtValue::List(v) => Self::write_list(buff, v)?,
                        NbtValue::Compound(name, v) => {
                            Self::write_compound(buff, name.as_ref(), v)?
                        }
                    }
                }
                buff.push(0);
                Ok(())
            }
            v => Err(NbtError::WrongRootType(v.tag()))
        }
    }

    #[inline]
    fn write_to_with_name(_name: &str, value: &crate::NbtValue, buff: &mut Vec<u8>) -> crate::error::NbtResult<()> {
        // drop name
        JavaNetty::write_to(value, buff)
    }

    #[inline]
    fn to_bytes(value: &NbtValue) -> NbtResult<Vec<u8>> {
        let mut buff = Vec::new();
        JavaNetty::write_to(value, &mut buff)?;
        Ok(buff)
    }

    fn write_text_component(writer: &mut Vec<u8>, value: &NbtValue) -> NbtResult<()> {
        match value {
            NbtValue::String(str) => Ok(Java::write_nbt_string(writer, str)),
            NbtValue::Compound(_, _) => JavaNetty::write_to(value, writer),
            x => Err(NbtError::WrongRootType(x.tag())),
        }
    }
}
