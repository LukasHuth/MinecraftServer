#![deny(missing_docs)]
//! This is a create for all the datatypes used across the whole project
use binary_utils::Result;
/// This trait is implemented by enums to be used in the artificial `Enum` struct
pub trait ImportantEnumTrait : Sized {
    /// This function creates a new instance of `Self` where data is the offset of the Option
    ///
    /// # Arguments
    ///
    /// `data` - id of the version
    ///
    /// # example
    /// 
    /// ```rust
    /// enum Example {
    ///     A,
    ///     B
    /// }
    /// impl ImportantEnumTrait for Example {
    ///     fn new(data: u64) -> Result<Self> {
    ///         match data {
    ///             0 => Ok(Self::A),
    ///             1 => Ok(Self::B),
    ///             _ => Err(Error::InvalidId),
    ///         }
    ///     }
    /// }
    /// #[test]
    /// fn test() {
    ///     let a = Example::new(0);
    ///     assert!(a, Ok(Example::A));
    ///     let b = Example::new(2);
    ///     assert!(b, Err(Error::InvalidId));
    /// }
    fn new(data: u64) -> Result<Self>;
}
mod basic_datatypes;
mod string_based;
mod complex_datatypes;
pub use basic_datatypes::*;
pub use string_based::*;
pub use complex_datatypes::*;
// INFO: the unused warnings are intended to see which datatype are missing implementations
/// A trait implemented by types to provide a `new` and `get_value` function
pub trait ImportantFunctions {
    /// The type needed to construct the type
    type InputType;
    /// The type returned by `get_value`
    type ReturnType;
    /// A function that constructs a new instance of the type
    fn new(data: Self::InputType) -> Self;
    /// A function that returns the hold data
    fn get_value(&self) -> Self::ReturnType;
}
/// A trait needed for enum operations, implemented by types that have to be converted to `u64`
pub trait GetU64 { 
    /// A function to get the hold value as `u64`
    fn get_u64(&self) -> u64; 
}
