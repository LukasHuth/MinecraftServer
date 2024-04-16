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
pub struct Boolean(bool);
/// A wrapper struct for a signed 8-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Byte(i8);
/// A wrapper struct for an unsigned 8-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
pub struct UnsignedByte(u8);
/// A wrapper struct for a signed 16-bit value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Short(i16);
/// A wrapper struct for an unsigned 16-bit value, typically used for implementing `DataReader` and `DataWriter`.
#[derive(Debug)]
pub struct UnsignedShort(u16);
/// A wrapper struct for a signed 32-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Int(i32);
/// A wrapper struct for a signed 64-bit integer value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Long(i64);
/// A wrapper struct for a 32-bit floating-point value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Float(f32);
/// A wrapper struct for a 64-bit floating-point value, typically used for implementing `DataReader` and `DataWriter`.
pub struct Double(f64);
mod implementations;
mod important_functions;
