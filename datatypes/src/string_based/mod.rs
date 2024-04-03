#[derive(Clone, Debug)]
pub struct String(std::string::String);
pub struct JSONTextComponent(String); // TODO: As JSON;
#[derive(Clone)]
pub struct Identifier(String);
mod implementations;
mod important_functions;
