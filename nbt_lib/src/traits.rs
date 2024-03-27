use crate::{error::NbtResult, NbtData, NbtValue};

pub trait NbtRead {
    fn read_i8(reader: &mut NbtData) -> NbtResult<i8>;
    fn read_i16(reader: &mut NbtData) -> NbtResult<i16>;
    fn read_i32(reader: &mut NbtData) -> NbtResult<i32>;
    fn read_i64(reader: &mut NbtData) -> NbtResult<i64>;
    fn read_f32(reader: &mut NbtData) -> NbtResult<f32>;
    fn read_f64(reader: &mut NbtData) -> NbtResult<f64>;
    fn read_i8_array(reader: &mut NbtData) -> NbtResult<Vec<i8>>;
    fn read_nbt_string(reader: &mut NbtData) -> NbtResult<String>;
    fn read_list(reader: &mut NbtData) -> NbtResult<Vec<NbtValue>>;
    fn read_compound(reader: &mut NbtData) -> NbtResult<Vec<(String, NbtValue)>>;
    fn read_i32_array(reader: &mut NbtData) -> NbtResult<Vec<i32>>;
    fn read_i64_array(reader: &mut NbtData) -> NbtResult<Vec<i64>>;
}
pub mod filesystem {
    use crate::{error::NbtResult, NbtValue, NbtData};

    pub fn read_root_compound(_reader: &mut NbtData) -> NbtResult<NbtValue> {
        todo!()
        // For example:
        // reader.read_compound
    }
}
pub mod network {
    pub mod pre764 {
        pub use super::super::filesystem::read_root_compound;
        // works exactly the same
    }
    pub mod after763 {
        use crate::{error::NbtResult, NbtValue, NbtData};
        // Since 1.20.2 (Protocol 764) NBT sent over the network has been updated to exclude the name from the root `TAG_COMPOUND`
        pub fn read_root_compound(_reader: &mut NbtData) -> NbtResult<NbtValue> {
            todo!()
        }
    }
}
