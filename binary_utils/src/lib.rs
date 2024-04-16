#![deny(missing_docs)]
//! This create provides important functions for reading and writing data
use std::future::Future;

use tokio::io::{AsyncWrite, AsyncRead};

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

/// Reads a single byte asyncronously from the provided reader
///
/// # Arguments
///
/// `reader` - A mutable reference to a type implemnting `AsyncRead` and `Unpin`
/// `line` - A line number, where the function is called
/// `file` - A static string slice containing the name of the file, where the function was called
///
/// # Returns
///
/// Returns an `binary_utils::Result` containing the byte or an error if the reading failed
///
/// # Errors
///
/// This function can fail, if there is no data in the provided `reader` to read.
pub async fn read_byte(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<u8> {
    let mut data = [0u8; 1];
    match reader.read_exact(&mut data).await {
        Ok(_) => Ok(data[0]),
        Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file, line)).into(),
    }
}
/// Consumes a single utf16be character asyncronously from the provided reader
///
/// This is intended to be used to move the cursor over an utf16be string.
///
/// # Arguments
///
/// `reader` - A mutable reference to a type implemnting `AsyncRead` and `Unpin`
/// `line` - A line number, where the function is called
/// `file` - A static string slice containing the name of the file, where the function was called
///
/// # Returns
///
/// Returns an `binary_utils::Result` containing `Ok(())` or an error if the reading failed
///
/// # Errors
///
/// This function can fail, if there is no data in the provided `reader` to read.
pub async fn consume_utf16be_char(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<()> {
    read_byte(reader, line, file).await?;
    read_byte(reader, line, file).await?;
    Ok(())
}
/// A function to asyncronously write data to the provided `writer`.
///
/// # Arguments
///
/// `writer` - A mutable reference to a type implementing `AsyncWrite` and `Unpin`
/// `bytes` - A reference to a slice of bytes that gets written into the provided `writer`
///
/// # Returns
///
/// Returns `Ok(())` or an error, if the write operation fails.
pub async fn write_bytes(writer: &mut (impl AsyncWrite + Unpin), bytes: &[u8]) -> Result<()> {
    match writer.write_all(bytes).await {
        Ok(_) => Ok(()),
        Err(_) => Error::FailedToWrite.into(),
    }
}
/// Reads a single UTF-8 character asyncronously from the provided reader
///
/// # Arguments
///
/// `reader` - A mutable reference to a type implemnting `AsyncRead` and `Unpin`
/// `line` - A line number, where the function is called
/// `file` - A static string slice containing the name of the file, where the function was called
///
/// # Returns
///
/// Returns an `binary_utils::Result` containing the UTF-8 `char` or an error if the reading failed
///
/// # Errors
///
/// This function can fail, if there is no data in the provided `reader` to read.
pub async fn read_utf8_char(reader: &mut (impl AsyncRead + Unpin), line: u32, file: &'static str) -> Result<char> {
    let current = read_byte(reader, line, file).await? as u8;
    match current {
        0b0000_0000..=0b0111_1111 => 
            match char::from_u32(current as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1100_0000..=0b1101_1111 => 
            match char::from_u32(((current as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1110_0000..=0b1110_1111 => 
            match char::from_u32(((current as u32) << 16) + ((read_byte(reader, line, file).await? as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        0b1111_0000..=0b1111_0111 => 
            match char::from_u32(((current as u32) << 24) + ((read_byte(reader, line, file).await? as u32) << 16) + ((read_byte(reader, line, file).await? as u32) << 8) + read_byte(reader, line, file).await? as u32) 
                { Some(v) => Ok(v), None => Error::InvalidStructure.into()},
        _ => Error::InvalidStructure.into()
    }
}
/// A trait for types that need to be read from a binary stream
///
/// Implementations of this trait provide a way to write their data 
/// asyncronously to a type that implements `AsyncRead` and `Unpin`
///
/// # Notes
///
/// This trait is intended to be implemented for types that represent structured data and provide
/// a method to asynchronously read that data from an input source. Implementations should handle
/// error conditions appropriately and return a `Result` indicating success or failure.
///
pub trait DataReader: Sized {
    // Reads data from the provided reader asynchronously and constructs an instance of `Self`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a type implementing `AsyncRead` and `Unpin` traits,
    ///              from which the data will be read.
    ///
    /// # Returns
    ///
    /// Returns a future representing the asynchronous reading operation. The future resolves to
    /// a `Result` containing an instance of `Self` if the operation is successful, or an error
    /// indicating failure.
    fn read(reader: &mut (impl AsyncRead + Unpin)) -> impl Future<Output = Result<Self>>;
}

/// A trait for types that have to read Packet data
pub trait PacketReader: Sized {
    /// Reads a packet from the provided stream `reader` using the specified `id` and `length`
    ///
    /// # Arguments
    ///
    /// `reader` - A mutable reference to a type implementing `AsyncRead` and `Unpin`
    /// `length` - the length of the packet (be careful, could be userdefined)
    /// `packet_id` - the id of the packet (packet type)
    fn read(
        reader: &mut (impl AsyncRead + Unpin),
        length: i32,
        packet_id: i32,
    ) -> impl Future<Output = Result<Self>>;
}

/// A trait for types that have to be writen to an asyncronous stream
///
/// # Notes
///
/// This trait is intended to be implemented for types that represent structured data and provide
/// a method to asynchronously write that data into an output source. Implementations should handle
/// error conditions appropriately and return a `Result` indicating success or failure.
///
pub trait DataWriter {
    /// Writes the data of the object into the defined `writer`
    ///
    /// # Arguments
    ///
    /// `writer` - A mutable reference to a type implementing `AsyncRead` and `Unpin`
    /// 
    /// # Returns
    ///
    /// Returns a future representing the asynchronous writing operation. The future resolves to
    /// a `Result` containing an instance of `()` if the operation is successful, or an error
    /// indicating failure.
    fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> impl Future<Output = Result<()>>;
}

/// A trait for types that need to be read from a binary stream but have an context based size
///
/// Implementations of this trait provide a way to write their data 
/// asyncronously to a type that implements `AsyncRead` and `Unpin`
///
/// # Notes
///
/// This trait is intended to be implemented for types that represent structured data and provide
/// a method to asynchronously read that data from an input source. Implementations should handle
/// error conditions appropriately and return a `Result` indicating success or failure.
///
pub trait ListDataReader: Sized {
    // Reads data from the provided reader asynchronously and constructs an instance of `Self`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a type implementing `AsyncRead` and `Unpin` traits,
    ///              from which the data will be read.
    /// * `length` - A length, how much data is contained, because the size is context based
    ///
    /// # Returns
    ///
    /// Returns a future representing the asynchronous reading operation. The future resolves to
    /// a `Result` containing an instance of `Self` if the operation is successful, or an error
    /// indicating failure.
    fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> impl Future<Output = Result<Self>>;
}
/// A trait implemented by every Packet to make them Storeable and recogniseable
pub trait Packet: Sized {}
/// An enum containing the different variants, why the read and write functions could fail
#[derive(Debug)]
pub enum Error {
    /// Used if an id was read that was not expected
    InvalidId,
    /// Used if the read structure dooes not fit the expected one
    InvalidStructure,
    /// Used if there are not enough bytes in the stream to read the expected data
    NotEnoughtBytes(String),
    /// Used if the Write Operation failed
    FailedToWrite,
}
/// A type alias for `std::result::Result` with the error type set to `Error`.
///
/// This type is commonly used throughout the codebase to represent the result of operations
/// that may fail, where `T` represents the success value and `Error` represents the type of error.
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    /// Converts the `Error` into a `Result` with an error value.
    ///
    /// This method is useful for converting an `Error` into a `Result` for functions
    /// or operations that return a `Result`.
    pub fn to_result<T>(self) -> Result<T> {
        Err(self)
    }
}
impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
