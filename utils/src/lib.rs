use std::io::{Read, Write};
pub enum ReadingError {
    FailedToRead,
    WrongIdentifier,
}
pub enum WritingError {
    FailedToWrite,
}
pub trait DataReader {
    fn read<T>(reader: &mut impl Read) -> std::result::Result<Self, T> where Self: Sized;
}
pub trait DataWriter {
    fn write<T>(&self, writer: &mut impl Write) -> std::result::Result<(), T>;
}
pub trait LengthedDataReader {
    fn read<T>(reader: &mut impl Read, length: u64) -> std::result::Result<Self, T> where Self: Sized;
}
