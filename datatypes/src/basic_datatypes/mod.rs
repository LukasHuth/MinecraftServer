pub struct Boolean(bool);
pub struct Byte(i8);
pub struct UnsignedByte(u8);
pub struct Short(i16);
#[derive(Debug)]
pub struct UnsignedShort(u16);
pub struct Int(i32);
pub struct Long(i64);
pub struct Float(f32);
pub struct Double(f64);
mod implementations;
mod important_functions;
