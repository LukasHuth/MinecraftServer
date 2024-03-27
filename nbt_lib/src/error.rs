use crate::NbtTypeId;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NbtError {
    UnknownErr(String),
    UnknownType(NbtTypeId),
    NameRead(String),
    VarIntTooBig(u128),
    VarLongTooBig(u128),
    ListTypeNotSame(NbtTypeId, NbtTypeId),
    IncorrectType(NbtTypeId, NbtTypeId),
    CursorOverflow(usize, usize, usize),
}

pub type NbtResult<T> = std::result::Result<T, NbtError>;

impl std::error::Error for NbtError {}

impl std::fmt::Display for NbtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::IncorrectType(t0, t1) => write!(f, "Incorrect type expected: {t0} found {t1}"),
            NbtError::NameRead(name) => write!(f, "failed to read: {name}"),
            NbtError::UnknownErr(err) => write!(f, "Encountered unknown error: {err}"),
            NbtError::UnknownType(type_id) => write!(f, "tried to read unknown type ({type_id})"),
            NbtError::VarIntTooBig(value) => write!(f, "Read a too big VarInt: {value}"),
            NbtError::VarLongTooBig(value) => write!(f, "Read a too big VarLong: {value}"),
            NbtError::ListTypeNotSame(t0, t1) => write!(f, "List types didn't match, expected: {t0} and found {t1}"),
            NbtError::CursorOverflow(c, l, a) => write!(f, "Tried to read out of bounce, data length is {a} and cursor is at {c} but a data length of {l} was requested"),
        }
    }
}
