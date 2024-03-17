use std::io::Read;

use crate::{tag_name::NBTTagName, utility::{rust_string_to_mutf8_bytes, read_byte}};
use utils::*;

pub struct NBTStringTag {
    pub data: String,
    pub tag_name: NBTTagName,
}
impl NBTStringTag {
    pub fn new(data: String, tag_name: String) -> Self {
        let tag_name = NBTTagName::new(tag_name);
        Self { data, tag_name}
    }
    pub fn as_mc_string(&self) -> Vec<u8> {
        rust_string_to_mutf8_bytes(&self.data)
    }
    pub fn to_net(&self) -> NetNBTStringTag {
        NetNBTStringTag { data: self.data.clone() }
    }
}
pub struct NetNBTStringTag {
    pub data: String,
}
impl NetNBTStringTag {
    pub fn new(data: String) -> Self {
        Self { data }
    }
    pub fn as_mc_string(&self) -> Vec<u8> {
        rust_string_to_mutf8_bytes(&self.data)
    }
    pub fn to_normal(&self, tag_name: String) -> NBTStringTag {
        NBTStringTag::new(self.data.clone(), tag_name)
    }
}
impl DataReader for NBTStringTag {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }?;
        if identifier != 0x01 { return Err(ReadingError::WrongIdentifier) }
        let tag_name = NBTTagName::read(reader)?;
        let data = NBTTagName::read(reader)?.name;
        Ok(Self{ data, tag_name })
    }
}
impl DataReader for NetNBTStringTag {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }?;
        if identifier != 0x01 { return Err(ReadingError::WrongIdentifier) }
        let data = NBTTagName::read(reader)?.name;
        Ok(Self{ data })
    }
}
impl DataWriter for NBTStringTag {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<(), WritingError> {
        match writer.write_all(&[0x08]) {
            Ok(_) => (),
            Err(_) => return Err(WritingError::FailedToWrite),
        }
        self.tag_name.write(writer)?;
        let data = NBTTagName::new(self.data.clone());
        data.write(writer)?;
        Ok(())
    }
}
impl DataWriter for NetNBTStringTag {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<(), WritingError> {
        match writer.write_all(&[0x08]) {
            Ok(_) => (),
            Err(_) => return Err(WritingError::FailedToWrite),
        }
        let data = NBTTagName::new(self.data.clone());
        data.write(writer)?;
        Ok(())
    }
}
