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
impl<T> ImportantFunctions for Array<T> where T: DataReader + DataWriter + Clone {
    type InputType = Vec<T>;

    type ReturnType = Vec<T>;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl ImportantFunctions for BitSet {
    type InputType = Vec<u64>;

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl ImportantFunctions for Angle {
    type InputType = u8;

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for Position {
    type InputType = (i32, i32, i16);

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data.0, data.1, data.2)
    }

    fn get_value(&self) -> Self::ReturnType {
        (self.0, self.1, self.2)
    }
}
impl ImportantFunctions for VarLong {
    type InputType = i64;

    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
