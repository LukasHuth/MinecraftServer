//! This module provides data structs for Region data
//!
//! # Data sources:
//! - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Region_file_format)
//! - [wiki.vg](https://wiki.vg/Map_Format)
use std::io::{BufWriter, Write};

use binary_utils::SyncDataWriter;

pub mod chunk;
use self::chunk::Chunk;

/// The location table of the Region
pub struct LocationAndTimestampTable {
    locations: [Location;1024],
    timestamps: [u32; 1024],
}
impl LocationAndTimestampTable {
    /// Returns the requested chunk of a region
    pub fn get(&self, x: isize, z: isize) -> Location {
        self.locations[(( x & 31) + ( z & 31) * 32) as usize]
    }
    /// Sets the location of a chunk
    pub fn set(&mut self, x: isize, z: isize, value: Location) {
        self.locations[(( x & 31) + ( z & 31) * 32) as usize] = value
    }
    /// Returns the requested timestamp of a region
    pub fn get_timestamp(&self, x: isize, z: isize) -> u32 {
        self.timestamps[(( x & 31) + ( z & 31) * 32) as usize]
    }
    /// Sets the timestamp of a chunk
    pub fn set_timestamp(&mut self, x: isize, z: isize, value: u32) {
        self.timestamps[(( x & 31) + ( z & 31) * 32) as usize] = value
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
impl Location {
    /// Converts the Location into a slice of bytes
    ///
    /// # Example
    /// ```rust
    /// use level_lib::anvil::region::Location;
    /// let loc = Location::new([0,0,1], 2);
    /// let loc_slice = loc.to_slice();
    /// assert_eq!(loc_slice, [0,0,1,2]);
    /// ```
    pub fn to_slice(&self) -> [u8; 4] {
        let mut res = [0u8; 4];
        res[..3].copy_from_slice(&self.offset);
        res[3] = self.size;
        res
    }
    /// creates a new instance of `Location`
    pub fn new(offset: [u8; 3], size: u8) -> Self {
        Self { offset, size }
    }
}
/// An enum of the different compression types
#[derive(Clone,Copy)]
#[repr(u8)]
pub enum CompressionScheme {
    /// Compressed with [gzip](https://en.wikipedia.org/wiki/Gzip)
    Gzip = 1,
    /// Compressed with [zlib](https://en.wikipedia.org/wiki/Zlib)
    Zlib = 2,
    /// Not compressed at all
    None = 3,
    /// Compressed with [lz4](https://github.com/lz4/lz4-java)
    LZ4 = 4,
}
/// A structure for region data
pub struct RegionData {
    timestamps: [u32; 1024],
    data: Vec<(isize, isize, Chunk)>,
}
const SECTOR_SIZE: usize = 4096;
/// Function to fill an vector until the start of the next sector is reached
///
/// # Test
/// ```
/// assert_eq!(0b1000000000000 - 1, 0b111111111111);
/// ```
fn fill_to_sector_end(input: &mut Vec<u8>) {
    let used_size = input.len() & (0b111111111111);
    if used_size == 0 { return; }
    let mut fill_data = vec![0; SECTOR_SIZE - used_size];
    input.append(&mut fill_data);
}
fn convert_u32_slice_to_u8_slice(data: &[u32; 1024]) -> [u8; 1024*4] {
    let mut res = [0; 1024*4];
    for i in 0..1024 {
        res[4*i..][..4].copy_from_slice(&data[i].to_be_bytes());
    }
    res
}
impl SyncDataWriter for LocationAndTimestampTable {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        let location_data = self.locations.map(|l|l.to_slice().to_vec()).to_vec().iter().flatten().map(|&v|v).collect::<Vec<u8>>();
        let location_data = location_data.as_slice();
        if let Err(_) = writer.write_all(&location_data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        let timestamp_data = convert_u32_slice_to_u8_slice(&self.timestamps);
        if let Err(_) = writer.write_all(&timestamp_data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
impl SyncDataWriter for RegionData {
    fn write(&self, writer: &mut impl Write) -> binary_utils::Result<()> {
        let mut latt = LocationAndTimestampTable::default();
        latt.set_timestamp_table(self.timestamps);
        let mut data: Vec<u8> = Vec::new();
        for (x, z, chunk) in self.data.iter() {
            let mut chunk_data = Vec::new();
            chunk.write(&mut chunk_data)?;
            let offset = data.len() / SECTOR_SIZE;
            let offset_data = offset.to_be_bytes()[0..3].to_vec();
            let offset = [offset_data[0], offset_data[1], offset_data[2]];
            let size = (chunk_data.len() / SECTOR_SIZE) as u8;
            let loc = Location { offset, size };
            latt.set(*x, *z, loc);
            let mut mut_chunk_data = chunk_data;
            data.append(&mut mut_chunk_data);
            fill_to_sector_end(&mut data);
        }
        let mut buf_writer = BufWriter::new(writer);
        latt.write(&mut buf_writer)?;
        if let Err(_) = buf_writer.write_all(&data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
