use crate::NbtTypeId;
/// Error enum to describe an error that occurs while reading/writing NBT data
#[derive(Debug, PartialEq, Eq)]
pub enum NbtError {
    /// Variant for unknown errors
    UnknownErr(String),
    /// This error occures if the root of the nbt data is not a `Compound`
    WrongRootType(NbtTypeId),
    /// This error occures if the root of the nbt data has no name but the name is required
    RootWithoutName,
    /// This error occures if the parsing reads and unknown type id
    UnknownType(NbtTypeId),
    /// This error occurs, if the parsing fails to read a name
    NameRead(String),
    /// This error occurs, if the parsing tries to read more data than supplied
    Overflow(usize, usize, usize),
    /// This error occurs, if a value is too large to be a `VarInt`, but a `VarInt` is expected
    VarIntTooBig(usize),
    /// This error occurs, if a value is too large to be a `VarLong`, but a `VarLong` is expected
    VarLongTooBig(usize),
    /// This error occurs, if elements in a list are not the same type
    ListTypeNotSame(Vec<NbtTypeId>),
    /// This error occurs, if a specific type is expected, but an other one is found
    IncorrectType(NbtTypeId, NbtTypeId),
}
/// A type declaration to store data `T` or return an error of `NbtError`
pub type NbtResult<T> = std::result::Result<T, NbtError>;
impl std::error::Error for NbtError {}
impl std::fmt::Display for NbtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::IncorrectType(t0, t1) => write!(f, "Expected type {t0} but got type {t1}"),
            NbtError::NameRead(name) => write!(f, "Could not read name '{name}'"),
            NbtError::Overflow(c, l, d) => write!(f, "Can't move cursor ({c}) by {l}, because the data length is only {d}"),
            NbtError::UnknownErr(err) => write!(f, "Unknown error occured: {err}"),
            NbtError::RootWithoutName => todo!(),
            NbtError::UnknownType(t0) => write!(f, "The type id {t0} does not corrospond to a valid type"),
            NbtError::VarIntTooBig(byte_length) => write!(f, "Could not construct VarInt, because the data would take {byte_length} bytes"),
            NbtError::VarLongTooBig(byte_length) => write!(f, "Could not construct VarLong, because the data would take {byte_length} bytes"),
            NbtError::ListTypeNotSame(types) => write!(f, "Could not create list, because following list types are not the same: {:?}", types),
            NbtError::WrongRootType(type_id) => write!(f, "The Root has to be an type id 0x0A, but is an {:02x}", type_id),
        }
    }
}

/// An error enum for nbt serialization
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Error {
    Message(String),
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedByte,
    ExpectedShort,
    ExpectedInteger,
    ExpectedLong,
    ExpectedFloat,
    ExpectedDouble,
    ExpectedByteArray,
    ExpectedIntArray,
    ExpectedLongArray,
    ExpectedString,
    ExpectedList,
    ExpectedMap,
    TrailingCharacters,
}
/// A Result type for nbt serialization
pub type NbtSerializeResult<T> = std::result::Result<T, Error>;
impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::Syntax => formatter.write_str("unexpected syntax"),
            Error::ExpectedBoolean => formatter.write_str("expected boolean"),
            Error::ExpectedByte => formatter.write_str("expected byte"),
            Error::ExpectedShort => formatter.write_str("expected short"),
            Error::ExpectedInteger => formatter.write_str("expected integer"),
            Error::ExpectedLong => formatter.write_str("expected long"),
            Error::ExpectedFloat => formatter.write_str("expected float"),
            Error::ExpectedDouble => formatter.write_str("expected double"),
            Error::ExpectedString => formatter.write_str("expected string"),
            Error::ExpectedList => formatter.write_str("expected list"),
            Error::ExpectedMap => formatter.write_str("expected map"),
            Error::ExpectedByteArray => formatter.write_str("expected byte array"),
            Error::ExpectedIntArray => formatter.write_str("expected int array"),
            Error::ExpectedLongArray => formatter.write_str("expected long array"),
            Error::TrailingCharacters => formatter.write_str("found training characters")
            /* and so forth */
        }
    }
}
impl std::error::Error for Error {}
impl serde::de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Error::Message(msg.to_string())
    }
}
impl serde::ser::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Error::Message(msg.to_string())
    }
}
