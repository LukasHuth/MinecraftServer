use super::*;
use crate::ImportantFunctions;

macro_rules! create_important_functions {
    ($($i:ident: $t:ty),*) => {
        $(
        impl ImportantFunctions for $i {
            type InputType = $t;

            type ReturnType = Self::InputType;

            fn new(data: Self::InputType) -> Self {
                Self(data)
            }

            fn get_value(&self) -> Self::ReturnType {
                self.0
            }

            fn set_value(&mut self, value: Self::InputType) {
                self.0 = value;
            }
        }
    )*
    };
}
create_important_functions!(
    Boolean: bool,
    Long: i64,
    Byte: i8,
    UnsignedByte: u8,
    Short: i16,
    UnsignedShort: u16,
    Int: i32,
    Float: f32,
    Double: f64
);
