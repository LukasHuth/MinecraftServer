use datatypes::{I26,I12};
pub struct Boolean(bool);
#[derive(Clone, Copy)]
pub struct Byte(i8);
pub struct UnsignedByte(u8);
pub struct Short(i16);
pub struct UnsignedShort(u16);
pub struct Int(i32);
#[derive(Clone, Copy)]
pub struct Long(i64);
pub struct Float(f32);
pub struct Double(f64);
pub struct String(VarInt, std::string::String);// TODO:
pub struct JSONTextComponent(std::string::String);// TODO:
pub struct Identifier(std::string::String);// TODO:
pub struct VarInt(u8,i32);// TODO:
pub struct VarLong(u8,i64);// TODO:
pub struct EntityMetadata();// TODO:
pub struct Slot(); // TODO:
pub struct NBT();// TODO:
pub struct Position(I26, I26, I12);
pub struct Angle(u8);
pub struct UUID(u64, u64); // FIRST MOST then LEAST
pub struct BitSet(VarInt, Vec<Long>); //TODO:
pub struct FixedBitSet(u64, Vec<Byte>); //TODO:
pub struct Optional(Option<u8>); //TODO: u8 is a placeholder
pub struct Array<I>(Vec<I>);
pub struct ByteArray(Vec<Byte>);
#[derive(Clone)]
pub struct Player {
    pub username: std::string::String,
    pub uuid: std::string::String,
}
impl Identifier {
    pub fn get_value(&self) -> std::string::String {
        self.0.clone()
    }
}
impl VarInt {
    pub fn new(bytes: u8, data: i32) -> Self {
        Self(bytes, data)
    }
    pub fn get_value(&self) -> i32 {
        self.1
    }
    pub fn get_bytes(&self) -> u8 {
        self.0
    }
}
impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        println!("value: {value}");
        let mut v = value;
        let mut n = 1;
        while v > 127 {
            v >>= 7;
            n+=1;
        }
        println!("n: {n}");
        Self::new(n, value)
    }
}
#[allow(unused_variables)]
pub mod necesary;
