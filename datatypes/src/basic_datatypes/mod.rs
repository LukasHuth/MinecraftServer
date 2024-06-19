use std::hash::Hash;

/// A wrapper struct for a boolean value, typically used for implementing `DataReader` and `DataWriter`.
/// # Examples
///
/// ```rust
/// use datatypes::Boolean;
/// use datatypes::ImportantFunctions;
///
/// let boolean_value = Boolean::new(true);
/// assert_eq!(boolean_value.get_value(), true);
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Boolean(bool);
/// A wrapper struct for a signed 8-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Byte(i8);
/// A wrapper struct for an unsigned 8-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Eq, Hash)]
pub struct UnsignedByte(u8);
/// A wrapper struct for a signed 16-bit value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Eq, Hash)]
pub struct Short(i16);
/// A wrapper struct for an unsigned 16-bit value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct UnsignedShort(u16);
/// A wrapper struct for a signed 32-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Int(i32);
/// A wrapper struct for a signed 64-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Long(i64);
/// A wrapper struct for a 32-bit floating-point value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Clone, Copy)]
pub struct Float(f32);
impl Eq for Float {
}
impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        i32::from_le_bytes(self.0.to_le_bytes()).hash(state)
    }
}
/// A wrapper struct for a 64-bit floating-point value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(PartialEq, Clone, Copy)]
pub struct Double(f64);
impl Eq for Double {
}
impl Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        i64::from_le_bytes(self.0.to_le_bytes()).hash(state)
    }
}
mod implementations;
mod important_functions;
