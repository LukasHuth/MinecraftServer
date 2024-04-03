use crate::ImportantFunctions;
use super::*;

impl ImportantFunctions for Long {
    type InputType = i64;

    type ReturnType = i64;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
impl ImportantFunctions for Boolean {
    type InputType = bool;

    type ReturnType = bool;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0
    }
}
