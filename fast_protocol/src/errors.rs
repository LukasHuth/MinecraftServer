#[derive(Debug)]
pub enum Error {
    InvalidId,
    InvalidStructure,
    NotEnoughtBytes,
    FailedToWrite,
}
pub type Result<T> = std::result::Result<T, Error>;
impl Error {
    pub fn to_result<T>(self) -> Result<T> {
        Err(self)
    }
}
impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
