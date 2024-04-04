use binary_utils::Result;
pub trait ImportantEnumTrait : Sized {
    fn new(data: u64) -> Result<Self>;
}
mod basic_datatypes;
mod string_based;
mod complex_datatypes;
pub use basic_datatypes::*;
pub use string_based::*;
pub use complex_datatypes::*;
// INFO: the unused warnings are intended to see which datatype are missing implementations
pub trait ImportantFunctions {
    type InputType;
    type ReturnType;
    fn new(data: Self::InputType) -> Self;
    fn get_value(&self) -> Self::ReturnType;
}
pub trait GetU64 { fn get_u64(&self) -> u64; }
