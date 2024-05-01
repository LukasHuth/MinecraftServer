use std::io::{Read, Cursor};

use flate2::read::GzDecoder;

use crate::error::{NbtResult, NbtError};

/// Struct that is needed for the NBT reading
pub struct NbtReader {
    /// A list of the whole data in bytes
    pub data: Vec<u8>,
    /// Cursor, where we are currently reading
    pub cursor: usize,
}

/// Module for version handling, because version 1.20.4 changed, how nbt data is send over the web
pub mod version_implementations;

impl NbtReader {
    /// function to create a new instance of `NbtReader`
    ///
    /// # Arguments
    /// `data` - List of the whole data in bytes
    pub fn new(data: Vec<u8>) -> NbtReader { Self { data, cursor: 0 } }
    /// function to create a new instance of `NbtReader` but before storing the byte array it gets
    /// decompressed
    ///
    /// # Arguments
    /// `data` - List of the whole data in bytes
    pub fn from_compressed_data(data: Vec<u8>) -> NbtReader {
        let cursor = Cursor::new(data);
        let mut decoder = GzDecoder::new(cursor);
        let mut data = Vec::new();
        if let Err(err) = decoder.read_to_end(&mut data) {
            println!("Error: {}", err);
        }
        // println!("{:?}", data);
        Self { data, cursor: 0 }
    }
    /// function to read an u8
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_u8(&mut self) -> NbtResult<u8> {
        if self.cursor + 1 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 1, self.data.len()))}
        let value = self.data[self.cursor];
        self.cursor += 1;
        Ok(value)
    }
    /// function to read an i8
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_i8(&mut self) -> NbtResult<i8> { Ok(self.read_u8()? as i8) }
    /// function to read an i16 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_be_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 2, self.data.len()))}
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor+1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// function to read an i16 LE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_le_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() { return Err(NbtError::Overflow(self.cursor, 2, self.data.len()))}
        let value = i16::from_le_bytes([self.data[self.cursor], self.data[self.cursor+1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// function to read an u16 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_be_u16(&mut self) -> NbtResult<u16> { Ok(self.read_be_i16()? as u16) }
    /// function to read an u16 LE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_le_u16(&mut self) -> NbtResult<u16> { Ok(self.read_le_i16()? as u16) }
    /// function to read an i32 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read an u32 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_be_u32(&mut self) -> NbtResult<u32> {
        Ok(self.read_be_i32()? as u32)
    }
    /// function to read a VarInt
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read a VarLong
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read a ZigZagVarInt
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_zigzag_var_i32(&mut self) -> NbtResult<i32> {
        let value = self.read_var_i32()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    /// function to read an i64 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read an u64 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_be_u64(&mut self) -> NbtResult<u64> { Ok(self.read_be_i64()? as u64) }
    /// function to read a f32 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_be_f32(&mut self) -> NbtResult<f32> { Ok(f32::from_bits(self.read_be_u32()?)) }
    /// function to read a f64 BE
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[inline]
    pub fn read_be_f64(&mut self) -> NbtResult<f64> { Ok(f64::from_bits(self.read_be_u64()?)) }
    /// function to read an u8 array
    ///
    /// # Arguments
    /// `len` - The amount of elements in the array
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_u8_array(&mut self, len: usize) -> NbtResult<Vec<u8>> {
        if self.cursor + len > self.data.len() { return Err(NbtError::Overflow(self.cursor, len, self.data.len()))}
        let value = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(value.to_vec())
    }
    /// function to read an i8 array
    ///
    /// # Arguments
    /// `len` - The amount of elements in the array
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
    #[cfg_attr(feature = "inline_read", inline)]
    pub fn read_i8_array(&mut self, len: usize) -> NbtResult<Vec<i8>> {
        Ok(self.read_u8_array(len)?.iter().map(|&v|v as i8).collect())
    }
    /// function to read an i32 BE array
    ///
    /// # Arguments
    /// `len` - The amount of elements in the array
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read an i64 BE array
    ///
    /// # Arguments
    /// `len` - The amount of elements in the array
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
    /// function to read an string
    ///
    /// # Arguments
    /// `len` - The length of the string
    ///
    /// # Returns
    ///
    /// Returns the value or an error, why whe read operation failed
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
