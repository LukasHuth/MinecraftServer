use crate::NbtTypeId;
#[derive(Debug, PartialEq, Eq)]
pub enum NbtError {
    UnknownErr(String),
    WrongRootType(NbtTypeId),
    RootWithoutName,
    UnknownType(NbtTypeId),
    NameRead(String),
    Overflow(usize, usize, usize),
    VarIntTooBig(usize),
    VarLongTooBig(usize),
    ListTypeNotSame(Vec<NbtTypeId>),
    IncorrectType(NbtTypeId, NbtTypeId),
}
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
