mod helpers;
use self::helpers::*;

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
pub struct Array(Vec<Datatype>);
pub struct Enum(Vec<Datatype>);
pub struct ByteArray(Vec<Byte>);
pub enum Datatype {
    Boolean(Boolean),
    Byte(Byte),
    UnsignedByte(UnsignedByte),
    Short(Short),
    UnsignedShort(UnsignedShort),
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    String(String),
    VarInt(VarInt),
    JSONTextComponent(JSONTextComponent),
    Identifier(Identifier),
    EntityMetadata(EntityMetadata),
    Slot(Slot),
    NBT(NBT),
    Angle(Angle),
    UUID(UUID),
    BitSet(BitSet),
    FixedBitSet(FixedBitSet),
    Optional(Optional),
    Array(Array),
    Enum(Enum),
    ByteArray(ByteArray),
}
pub struct CompressedPacket {
    packet_length: VarInt,
    data_length: VarInt,
    packet_id: Option<VarInt>,
    data: Option<ByteArray>,
}
pub struct UncompressedPacket {
    length: VarInt,
    packet_id: VarInt,
    data: ByteArray,
}
pub enum Packet {
    Compressed(CompressedPacket),
    Uncompressed(UncompressedPacket)
}
pub mod validation;
pub mod necesary;
pub mod packet_implementation;
