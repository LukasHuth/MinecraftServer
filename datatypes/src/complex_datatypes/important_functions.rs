use crate::ImportantFunctions;
use super::*;

impl ImportantFunctions for VarInt {
    type InputType = i32;

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for UUID {
    type InputType = u128;

    type ReturnType = u128;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for ByteArray {
    type InputType = Vec<u8>;

    type ReturnType = Vec<u8>;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl<T, S> ImportantFunctions for Enum<T, S> where S: DataReader + GetU64, T: ImportantEnumTrait + Clone {
    type InputType = (T, S);

    type ReturnType = T;

    fn new(data: Self::InputType) -> Self {
        Self(data.0, data.1)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
