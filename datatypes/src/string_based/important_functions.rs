use crate::ImportantFunctions;

use super::*;

impl ImportantFunctions for String {
    type InputType = std::string::String;

    type ReturnType = std::string::String;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }

    fn set_value(&mut self, value: Self::InputType) {
        self.0 = value;
    }
}
impl ImportantFunctions for Identifier {
    type InputType = std::string::String;
    type ReturnType = std::string::String;
    fn new(data: Self::InputType) -> Self {
        Self(String::new(data))
    }
    fn get_value(&self) -> Self::ReturnType {
        self.0.get_value()
    }

    fn set_value(&mut self, value: Self::InputType) {
        self.0 = String::new(value);
    }
}
