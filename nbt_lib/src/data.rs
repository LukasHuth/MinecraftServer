use std::io::Read;

use flate2::{read::{GzDecoder, GzEncoder}, Compression};
use tokio::io::{AsyncRead, AsyncReadExt as _};

use crate::error::{NbtResult, NbtError};

pub type NbtTypeId = u8;
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>), // length prefixed: i32,
    String(String), // length prefixed: u16 & modified utf8
    List(Vec<NbtValue>),
    Compound(Option<String>, Vec<(String, NbtValue)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}
mod nbt_value_trait_implementation;
pub struct NbtData {
    data: Vec<u8>,
    cursor: usize,
}
impl NbtData {
    pub fn new_with_array(data: Vec<u8>) -> Self {
        let cursor = 0;
        Self { data, cursor }
    }
    pub async fn new(data: &mut (impl AsyncRead + Unpin)) -> NbtResult<Self> {
        let mut new_data = Vec::new();
        match data.read_to_end(&mut new_data).await {
            Ok(_) => Ok(()),
            Err(err) => Err(NbtError::UnknownErr(format!("{}", err))),
        }?;
        let data = new_data;
        let cursor = 0;
        Ok(Self { data, cursor })
    }
    pub fn from_uncompressed_data(data: Vec<u8>) -> NbtResult<Self> {
        let mut decoder = GzDecoder::new(&data[..]);
        let mut uncompressed_data = Vec::new();
        match decoder.read_to_end(&mut uncompressed_data) {
            Ok(_) => (),
            Err(err) => return Err(NbtError::UnknownErr(format!("{}", err))),
        }
        let data = uncompressed_data;
        let cursor = 0;
        Ok(Self { data, cursor })
    }
    pub fn compressed_data(&mut self) -> NbtResult<Vec<u8>> {
        let mut encoder = GzEncoder::new(&self.data[..], Compression::default());
        let mut compressed_data = Vec::new();
        match encoder.read_to_end(&mut compressed_data) {
            Ok(_) => (),
            Err(err) => return Err(NbtError::UnknownErr(format!("{}", err))),
        }
        Ok(compressed_data)
    }
    pub fn read_u8(&mut self) -> NbtResult<u8> {
        let mut data = [0;1];
        match self.read(&mut data) {
            Ok(_) => (),
            Err(err) => return Err(NbtError::UnknownErr(format!("{err}"))),
        }
        let result = u8::from_be_bytes(data);
        Ok(result)
    }
    pub fn read_be_u16(&mut self) -> NbtResult<u16> {
        let mut data = [0;2];
        if let Err(err) = self.read(&mut data) {
            return Err(NbtError::UnknownErr(format!("{err}")))
        }
        let result = u16::from_be_bytes(data);
        Ok(result)
    }
    pub fn read_be_u32(&mut self) -> NbtResult<u32> {
        let mut data = [0;4];
        if let Err(err) = self.read(&mut data) {
            return Err(NbtError::UnknownErr(format!("{err}")))
        }
        let result = u32::from_be_bytes(data);
        Ok(result)
    }
    pub fn read_be_u64(&mut self) -> NbtResult<u64> {
        let mut data = [0;8];
        if let Err(err) = self.read(&mut data) {
            return Err(NbtError::UnknownErr(format!("{err}")))
        }
        let result = u64::from_be_bytes(data);
        Ok(result)
    }
    pub fn read_var_i32(&mut self) -> NbtResult<i32> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as i32) << (size * 7);
            size += 1;
            if size > 5 {
                return Err(NbtError::VarIntTooBig(value as u128));
            }
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(value)
    }
    pub fn read_var_i64(&mut self) -> NbtResult<i64> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as i64) << (size * 7);
            size += 1;
            if size > 10 {
                return Err(NbtError::VarLongTooBig(value as u128));
            }
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(value)
    }
    pub fn read_zigzag_var_i32(&mut self) -> NbtResult<i32> {
        let value = self.read_var_i32()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    pub fn read_zigzag_var_i64(&mut self) -> NbtResult<i64> {
        let value = self.read_var_i64()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    pub fn read_be_f32(&mut self) -> NbtResult<f32> {
        let data = self.read_be_u32()?;
        Ok(f32::from_bits(data))
    }
    pub fn read_be_f64(&mut self) -> NbtResult<f64> {
        let data = self.read_be_u64()?;
        Ok(f64::from_bits(data))
    }
    pub fn read_i8_array(&mut self, len: usize) -> NbtResult<Vec<i8>> {
        if self.cursor + len > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let data = self.data[self.cursor..self.cursor+len].iter().map(|&i|i as i8).collect();
        self.cursor+=len;
        Ok(data)
    }
    pub fn read_i32_array(&mut self, len: usize) -> NbtResult<Vec<i32>> {
        if self.cursor + len * 4 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let data = self.data[self.cursor..self.cursor+len*4].chunks_exact(4).map(|i|i32::from_be_bytes(i[0..4].try_into().unwrap())).collect();
        self.cursor+=len;
        Ok(data)
    }
    pub fn read_i64_array(&mut self, len: usize) -> NbtResult<Vec<i64>> {
        if self.cursor + len * 8 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let data = self.data[self.cursor..self.cursor+len*8].chunks_exact(8).map(|i|i64::from_be_bytes(i[0..8].try_into().unwrap())).collect();
        self.cursor+=len;
        Ok(data)
    }
    pub fn read_string(&mut self, len: usize) -> NbtResult<String> {
        if len + self.cursor > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]).to_string();
        self.cursor += len;
        Ok(value)
    }
}
impl Read for NbtData {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let available_data = self.data.len() - self.cursor;
        if available_data == 0 { return Ok(0); }

        let bytes_to_copy = std::cmp::min(buf.len(), available_data);
        buf[..bytes_to_copy].copy_from_slice(&self.data[self.cursor..self.cursor + bytes_to_copy]);
        self.cursor += bytes_to_copy;

        Ok(bytes_to_copy)
    }
}
