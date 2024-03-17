use std::io::Read;

use crate::{tag_name::NBTTagName, utility::read_byte};

use utils::*;

pub struct NBTByteTag {
    pub data: i8,
    tag_name: NBTTagName,
}
pub struct NetNBTByteTag {
    pub data: i8,
}
impl NBTByteTag {
    pub fn new(data: i8, tag_name: String) -> Self {
        let tag_name = NBTTagName::new(tag_name);
        Self { data, tag_name }
    }
    pub fn to_net(&self) -> NetNBTByteTag {
        NetNBTByteTag { data: self.data }
    }
}
impl NetNBTByteTag {
    pub fn new(data: i8) -> Self {
        Self { data }
    }
    pub fn to_normal(&self, name: String) -> NBTByteTag {
        NBTByteTag { data: self.data, tag_name: NBTTagName::new(name)}
    }
}
impl DataReader for NBTByteTag {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }?;
        if identifier != 0x01 { return Err(ReadingError::WrongIdentifier) }
        let tag_name = NBTTagName::read(reader)?;
        let data = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }? as i8;
        Ok(Self{ data, tag_name })
    }
}
impl DataReader for NetNBTByteTag {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }?;
        if identifier != 0x01 { return Err(ReadingError::WrongIdentifier) }
        let data = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }? as i8;
        Ok(Self{ data })
    }
}
impl DataWriter for NBTByteTag {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<(), WritingError> {
        match writer.write_all(&[0x01]) {
            Ok(_) => (),
            Err(_) => return Err(WritingError::FailedToWrite),
        };
        self.tag_name.write(writer)?;
        match writer.write_all(&[self.data as u8]) {
            Ok(_) => Ok(()),
            Err(_) => return Err(WritingError::FailedToWrite),
        }
    }
}
impl DataWriter for NetNBTByteTag {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<(), WritingError> {
        match writer.write_all(&[0x01]) {
            Ok(_) => (),
            Err(_) => return Err(WritingError::FailedToWrite),
        };
        match writer.write_all(&[self.data as u8]) {
            Ok(_) => Ok(()),
            Err(_) => return Err(WritingError::FailedToWrite),
        }
    }
}
