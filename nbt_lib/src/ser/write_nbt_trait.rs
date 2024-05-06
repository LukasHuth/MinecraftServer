use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use crate::{error::{Error, Result}, NbtTypeId};

pub trait WriteNbt: Write {
    fn write_tag(&mut self, tag: NbtTypeId) -> Result<()> {
        match self.write_u8(tag as u8) {
            Err(_) => Err(Error::Message("writing failed".to_string())),
            Ok(()) => Ok(()),
        }
    }
    fn write_nbt_string(&mut self, key: &str) -> Result<()> {
        let key = cesu8::to_java_cesu8(key);
        let key_length = key.len() as u16;
        self.write_u16_be(key_length)?;
        if let Err(_) = self.write_all(&key) {
            return Err(Error::Message("writing failed".to_string()));
        }
        Ok(())
    }
    fn write_len(&mut self, len: usize) -> Result<()> {
        if len > i32::MAX as usize { return Err(Error::Message("Too large length".to_string())) }
        if let Err(_) = self.write_u32::<BigEndian>(len as u32) {
            return Err(Error::Message("writing failed".to_string()));
        }
        Ok(())
    }
    fn write_u16_be(&mut self, value: u16) -> Result<()> {
        match self.write_u16::<BigEndian>(value) {
            Err(_) => Err(Error::Message("writing failed".to_string())),
            Ok(()) => Ok(()),
        }
    }
    fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        match self.write_all(value) {
            Err(_) => Err(Error::Message("writing failed".to_string())),
            Ok(()) => Ok(()),
        }
    }
}

impl<T> WriteNbt for T where T: Write {}
