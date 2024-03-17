use std::io::Read;

use utils::{DataReader, ReadingError, DataWriter};
use crate::utility::{rust_string_to_mutf8_bytes, read_char, mutf8_bytes_to_rust_string};

// use crate::utils;

pub struct NBTTagName {
    pub name: String,
    length: usize,
}
impl NBTTagName {
    pub fn new(name: String) -> Self {
        Self { length: rust_string_to_mutf8_bytes(&name).len() , name }
    }
    pub fn get_length(&self) -> usize {
        self.length
    }
}
impl DataReader for NBTTagName {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> {
        let mut data = [0;2];
        match reader.read_exact(&mut data) {
            Ok(_) => (),
            Err(_) => return Err(ReadingError::FailedToRead),
        }
        let length: u16 = ((data[0] as u16) << 8) | data[1] as u16;
        let length = length as usize;
        let mut data = Vec::new();
        for _ in 0..length {
            let mut char = match read_char(reader) {
                Ok(v) => v,
                Err(_) => return Err(ReadingError::FailedToRead),
            };
            data.append(&mut char);
        }
        let name = mutf8_bytes_to_rust_string(&data);
        Ok(Self { name, length })
    }
}
impl DataWriter for NBTTagName {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> Result<(), utils::WritingError> {
        let length: [u8;2] = [((self.length >> 8) & 0xFF) as u8, (self.length & 0xFF) as u8];
        match writer.write_all(&length) {
            Ok(_) => Ok(()),
            Err(_) => Err(utils::WritingError::FailedToWrite),
        }?;
        match writer.write_all(&rust_string_to_mutf8_bytes(&self.name)) {
            Ok(_) => Ok(()),
            Err(_) => Err(utils::WritingError::FailedToWrite),
        }?;
        Ok(())
    }
}
