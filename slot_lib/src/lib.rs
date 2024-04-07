pub mod datatypes;
pub enum Slot {
    Empty,
    Data(::datatypes::VarInt,::datatypes::Byte, Option<nbt_lib::datatypes::NBT>)
}
