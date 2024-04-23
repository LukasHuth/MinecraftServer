//! Module containing structs to save data in the Anvil format
//!
//! More informations [here](https://minecraft.fandom.com/wiki/Anvil_file_format)

use std::io::Write;

use binary_utils::{DataReader, SyncDataWriter};
use nbt_lib::version::Java;

/// The location table of the Region
pub struct LocationAndTimestampTable {
    locations: [Location;1024],
    timestamps: [u32; 1024],
}
impl LocationAndTimestampTable {
    /// Returns the requested chunk of a region
    pub fn get(&self, x: usize, z: usize) -> Location {
        self.locations[(x % 32) + (z % 32) * 32]
    }
    /// Sets the location of a chunk
    pub fn set(&mut self, x: usize, z: usize, value: Location) {
        self.locations[(x % 32) + (z % 32) * 32] = value
    }
    /// Returns the requested timestamp of a region
    pub fn get_timestamp(&self, x: usize, z: usize) -> u32 {
        self.timestamps[(x % 32) + (z % 32) * 32]
    }
    /// Sets the timestamp of a chunk
    pub fn set_timestamp(&mut self, x: usize, z: usize, value: u32) {
        self.timestamps[(x % 32) + (z % 32) * 32] = value
    }
    /// Set the timestamp table
    pub fn set_timestamp_table(&mut self, value: [u32; 1024]) {
        self.timestamps = value;
    }
}
impl Default for LocationAndTimestampTable {
    fn default() -> Self {
        let locations = [Location { offset: [0;3], size: 0}; 1024];
        let timestamps = [0; 1024];
        Self { locations, timestamps }
    }
}
/// Location data
#[derive(Clone, Copy)]
pub struct Location {
    /// The offset of the chunk
    pub offset: [u8; 3],
    /// The size of the data
    pub size: u8
}
/// An enum of the different compression types
#[derive(Clone,Copy)]
#[repr(u8)]
pub enum CompressionScheme {
    /// Compressed with [gzip](https://en.wikipedia.org/wiki/Gzip)
    Gzip = 1,
    /// Compressed with [zlib](https://en.wikipedia.org/wiki/Zlib)
    Zlib = 2,
}
/// A struct containing the chunk data
pub struct Chunk {
    /// length of the data in bytes
    pub length: u32,
    /// The used compression
    pub compression: CompressionScheme,
    /// The compressed data as nbt data
    pub data: nbt_lib::NbtValue,
}
impl SyncDataWriter for Chunk {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        if let Err(_) = writer.write_all(&self.length.to_be_bytes()) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        if let Err(_) = writer.write_all(&[self.compression as u8]) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        let data = self.data.to_binary::<Java>();
        // TODO: Compress the data
        let data = if let Ok(data) = data { data } else { return Err(binary_utils::Error::FailedToWrite); };
        if let Err(_) = writer.write_all(&data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
/// A structure for region data
pub struct RegionData {
    timestamps: [u32; 1024],
    data: Vec<(u8, u8, Chunk)>,
}
impl SyncDataWriter for RegionData {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        let mut latt = LocationAndTimestampTable::default();
        latt.set_timestamp_table(self.timestamps);
        let mut data: Vec<u8> = Vec::new();
        for (x, z, chunk) in self.data.iter() {
            let mut chunk_data = Vec::new();
            chunk.write(&mut chunk_data)?;
        }
        Ok(())
    }
}
