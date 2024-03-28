use std::io::{Read, Cursor};

use flate2::read::GzDecoder;

use crate::error::{NbtResult, NbtError};

pub struct NbtReader {
    pub data: Vec<u8>,
    pub cursor: usize,
}

pub mod version_implementations;

impl NbtReader {
    pub fn new(data: Vec<u8>) -> NbtReader { Self { data, cursor: 0 } }
    pub fn from_compressed_data(data: Vec<u8>) -> NbtReader {
        let cursor = Cursor::new(data);
        let mut decoder = GzDecoder::new(cursor);
        let mut data = Vec::new();
        if let Err(err) = decoder.read_to_end(&mut data) {
            println!("Error: {}", err);
        }
        println!("{:?}", data);
        Self { data, cursor: 0 }
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_u8(&mut self) -> NbtResult<u8> {
        if self.cursor + 1 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 1, self.data.len()))}
        let value = self.data[self.cursor];
        self.cursor += 1;
        Ok(value)
    }
    #[inline]
    pub fn read_i8(&mut self) -> NbtResult<i8> { Ok(self.read_u8()? as i8) }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 2, self.data.len()))}
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor+1]]);
        self.cursor += 2;
        Ok(value)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_le_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 2, self.data.len()))}
        let value = i16::from_le_bytes([self.data[self.cursor], self.data[self.cursor+1]]);
        self.cursor += 2;
        Ok(value)
    }
    #[inline]
    pub fn read_be_u16(&mut self) -> NbtResult<u16> { Ok(self.read_be_i16()? as u16) }
    #[inline]
    pub fn read_le_u16(&mut self) -> NbtResult<u16> { Ok(self.read_le_i16()? as u16) }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i32(&mut self) -> NbtResult<i32> {
        if self.cursor + 4 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 4, self.data.len()))}
        let value = i32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }
    #[inline]
    pub fn read_be_u32(&mut self) -> NbtResult<u32> {
        Ok(self.read_be_i32()? as u32)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_var_i32(&mut self) -> NbtResult<i32> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0b0111_1111) as i32) << (size * 7);
            size += 1;
            if size > 5 {
                return Err(NbtError::VarIntTooBig(value as usize));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(value)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_var_i64(&mut self) -> NbtResult<i64> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0b0111_1111) as i64) << (size * 7);
            size += 1;
            if size > 10 {
                return Err(NbtError::VarIntTooBig(value as usize));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(value)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_zigzag_var_i32(&mut self) -> NbtResult<i32> {
        let value = self.read_var_i32()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i64(&mut self) -> NbtResult<i64> {
        if self.cursor + 8 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 8, self.data.len()))}
        let value = i64::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
            self.data[self.cursor + 4],
            self.data[self.cursor + 5],
            self.data[self.cursor + 6],
            self.data[self.cursor + 7],
        ]);
        self.cursor += 8;
        Ok(value)
    }
    #[inline]
    pub fn read_be_u64(&mut self) -> NbtResult<u64> { Ok(self.read_be_i64()? as u64) }
    #[inline]
    pub fn read_be_f32(&mut self) -> NbtResult<f32> { Ok(f32::from_bits(self.read_be_u32()?)) }
    #[inline]
    pub fn read_be_f64(&mut self) -> NbtResult<f64> { Ok(f64::from_bits(self.read_be_u64()?)) }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_u8_array(&mut self, len: usize) -> NbtResult<Vec<u8>> {
        if self.cursor + len > self.data.len() { return Err(NbtError::Overflow(self.cursor, len, self.data.len()))}
        let value = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(value.to_vec())
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_i8_array(&mut self, len: usize) -> NbtResult<Vec<i8>> {
        Ok(self.read_u8_array(len)?.iter().map(|&v|v as i8).collect())
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i32_array(&mut self, len: usize) -> NbtResult<Vec<i32>> {
        if self.cursor + len * 4 > self.data.len() { return Err(NbtError::Overflow(self.cursor, len, self.data.len()))}
        let value = self.data[self.cursor..self.cursor + len * 4]
            .chunks_exact(4)
            .map(|n| i32::from_be_bytes(n[0..4].try_into().unwrap()))
            .collect();
        self.cursor += len * 4;
        Ok(value)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i64_array(&mut self, len: usize) -> NbtResult<Vec<i64>> {
        if self.cursor + len * 8 > self.data.len() { return Err(NbtError::Overflow(self.cursor, len, self.data.len()))}
        let value = self.data[self.cursor..self.cursor + len * 8]
            .chunks_exact(8)
            .map(|n| i64::from_be_bytes(n[0..8].try_into().unwrap()))
            .collect();
        self.cursor += len * 8;
        Ok(value)
    }
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_string(&mut self, len: usize) -> Result<String, NbtError> {
        if len + self.cursor > self.data.len() {
            return Err(NbtError::Overflow(self.cursor, len, self.data.len()));
        }
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]);
        self.cursor += len;
        Ok(value.into_owned())
    }
}
