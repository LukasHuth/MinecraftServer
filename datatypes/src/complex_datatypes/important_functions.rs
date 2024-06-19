use super::*;
use crate::{ImportantFunctions, TypedImportantFunctions};

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
impl<T, S> ImportantFunctions for Enum<T, S>
where
    S: DataReader + GetU64,
    T: ImportantEnumTrait + Clone,
{
    type InputType = (T, S);

    type ReturnType = T;

    fn new(data: Self::InputType) -> Self {
        Self(data.0, data.1)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl<T, const S: u64> ImportantFunctions for FixedPoint<T, S>
where
    T: GetU64 + ImportantFunctions,
    <T as ImportantFunctions>::InputType: From<u64>
{
    type InputType = f64;

    type ReturnType = f64;

    fn new(data: f64) -> Self where <T as ImportantFunctions>::InputType: From<u64>{
        let data: u64 = (data * S as f64) as u64;
        let data: T = T::new(data.into());
        Self(data)
    }

    fn get_value(&self) -> f64 {
        (self.0.get_u64() as f64) / S as f64
    }
}
impl<T> ImportantFunctions for Array<T>
where
    T: DataReader + DataWriter + Clone,
{
    type InputType = Vec<T>;

    type ReturnType = Vec<T>;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
impl TypedImportantFunctions<Vec<u64>> for BitSet {
    fn new(data: Vec<u64>) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Vec<u64> {
        self.0.clone()
    }
}
impl TypedImportantFunctions<Vec<bool>> for BitSet {
    fn new(data: Vec<bool>) -> Self {
        let data: Vec<u64> = data
            .chunks(64)
            .map(|bits| {
                bits.into_iter().enumerate().fold(
                    0u64,
                    |acc, (i, &bit)| {
                        if bit {
                            acc | (1 << i)
                        } else {
                            acc
                        }
                    },
                )
            })
            .collect();
        Self::new(data)
    }

    fn get_value(&self) -> Vec<bool> {
        let data = self
            .0
            .iter()
            .map(|&value| {
                let mut bits = [false; 64];
                for i in 0..64 {
                    bits[i] = if get_nth_bit(value, i) == 1 {
                        true
                    } else {
                        false
                    };
                }
                bits
            })
            .flatten()
            .collect();
        data
    }
}
fn get_nth_bit(number: u64, n: usize) -> u64 {
    (number >> n) & 1
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
