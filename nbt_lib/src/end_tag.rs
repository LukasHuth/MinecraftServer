use std::io::{Read, Write};

use utils::*;
use crate::utility::read_byte;

pub struct NBTEndTag {}
impl DataReader for NBTEndTag {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier = match read_byte(reader) {
            Ok(v) => Ok(v),
            Err(_) => Err(ReadingError::FailedToRead),
        }?;
        if identifier != 0x01 { return Err(ReadingError::WrongIdentifier) }
        Ok(Self{ })
    }
}
impl DataWriter for NBTEndTag {
    fn write(&self, writer: &mut impl Write) -> Result<(), WritingError> {
        match writer.write(&[0]) {
            Ok(_) => Ok(()),
            Err(_) => Err(WritingError::FailedToWrite)
        }
    }
}
