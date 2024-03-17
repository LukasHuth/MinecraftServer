use std::io::{Read, Write};
pub enum ReadingError {
    FailedToRead,
    WrongIdentifier,
}
pub enum WritingError {
    FailedToWrite,
}
pub trait DataReader {
    fn read(reader: &mut impl Read) -> Result<Self, ReadingError> where Self: Sized;
}
pub trait DataWriter {
    fn write(&self, writer: &mut impl Write) -> Result<(), WritingError>;
}
pub trait LengthedDataReader {
    fn read(reader: &mut impl Read, length: u64) -> Result<Self, ReadingError> where Self: Sized;
}
