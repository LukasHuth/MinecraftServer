//! This Module provides datastructures for chunk data
//!
//! # Sources
//! - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Chunk_format)

use std::io::{Cursor, Read as _, Write};

use binary_utils::SyncDataWriter;
use flate2::{read::{GzEncoder, ZlibEncoder}, Compression};
use nbt_lib::{traits::IntoNbt, version::Java};

use self::chunk_data::ChunkData;

use super::CompressionScheme;

pub mod block_entity;
pub mod section;
pub mod chunk_data;

/// A struct containing the chunk data
pub struct Chunk {
    /// length of the data in bytes
    pub length: u32,
    /// The used compression
    pub compression: CompressionScheme,
    /// The compressed data as nbt data
    pub data: ChunkData,
}
impl SyncDataWriter for Chunk {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        if let Err(_) = writer.write_all(&self.length.to_be_bytes()) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        if let Err(_) = writer.write_all(&[self.compression as u8]) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        let data = self.data.to_nbt().to_binary::<Java>();
        let data = if let Ok(data) = data { data } else { return Err(binary_utils::Error::FailedToWrite); };
        let mut compressed_data: Vec<u8> = Vec::new();
        match self.compression {
            CompressionScheme::None => compressed_data = data,
            CompressionScheme::LZ4 => todo!(),
            CompressionScheme::Gzip => {
                let cursor = Cursor::new(data);
                let mut encoder = GzEncoder::new(cursor, Compression::default());
                if let Err(_) = encoder.read(&mut compressed_data) {
                    return Err(binary_utils::Error::FailedToWrite);
                }
            }
            CompressionScheme::Zlib => {
                let cursor = Cursor::new(data);
                let mut encoder = ZlibEncoder::new(cursor, Compression::default());
                if let Err(_) = encoder.read(&mut compressed_data) {
                    return Err(binary_utils::Error::FailedToWrite);
                }
            }
        }
        if let Err(_) = writer.write_all(&compressed_data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
