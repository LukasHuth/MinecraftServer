use datatypes::{I26,I12};
use derives::TestNeccessaryTrait;
use uuid::Uuid;
use crate::TestNeccessaryTrait;
#[derive(Clone, TestNeccessaryTrait)]
pub struct Boolean(bool);
#[derive(Clone, Copy, TestNeccessaryTrait)]
pub struct Byte(i8);
#[derive(TestNeccessaryTrait)]
pub struct UnsignedByte(u8);
#[derive(TestNeccessaryTrait)]
pub struct Short(i16);
#[derive(TestNeccessaryTrait)]
pub struct UnsignedShort(u16);
#[derive(TestNeccessaryTrait)]
pub struct Int(i32);
#[derive(Clone, Copy, TestNeccessaryTrait)]
pub struct Long(i64);
#[derive(TestNeccessaryTrait)]
pub struct Float(f32);
#[derive(TestNeccessaryTrait)]
pub struct Double(f64);
#[derive(Clone, TestNeccessaryTrait)]
pub struct String{
    data: std::string::String
}
#[derive(TestNeccessaryTrait)]
pub struct TextComponent{
    tag: NBTTag,
}
#[derive(TestNeccessaryTrait)]
pub struct JSONTextComponent(std::string::String);// TODO:
#[derive(TestNeccessaryTrait)]
pub struct Identifier(std::string::String);// TODO:
#[derive(Clone, TestNeccessaryTrait)]
pub struct VarInt {
    data: i32
}
#[derive(TestNeccessaryTrait)]
pub struct VarLong(i64);
pub struct EntityMetadata();// TODO:
pub struct Slot(); // TODO:
pub struct NBT();// TODO:
pub struct Position(I26, I26, I12);
#[derive(TestNeccessaryTrait)]
pub struct Angle(u8);
#[derive(Clone, TestNeccessaryTrait)]
pub struct UUID(u128); // FIRST MOST then LEAST
pub struct BitSet(VarInt, Vec<Long>); //TODO:
pub struct FixedBitSet(u64, Vec<Byte>); //TODO:
#[derive(TestNeccessaryTrait)]
pub struct Optional(Option<u8>); //TODO: u8 is a placeholder
pub struct Array<I>(Vec<I>);
#[derive(TestNeccessaryTrait)]
pub struct ByteArray(Vec<Byte>);
pub mod player;
impl UUID {
    pub fn to_string(&self) -> std::string::String {
        uuid::Uuid::from_u128(self.0).hyphenated().to_string()
    }
    pub fn to_uuid(&self) -> Uuid {
        uuid::Uuid::from_u128(self.0)
    }
}
impl Identifier {
    pub fn get_value(&self) -> std::string::String {
        self.0.clone()
    }
}
impl VarInt {
    pub fn get_value(&self) -> i32 {
        self.data
    }
    pub fn get_bytes(&self) -> u8 {
        let mut result = 1;
        let mut value = self.data as u128;
        while value > 0x7F {
            value >>= 7;
            result += 1;
        }
        result
    }
}
#[allow(unused_variables)]
pub mod necesary;
