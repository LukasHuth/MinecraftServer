use binary_utils::{DataReader, DataWriter};

use crate::{ImportantEnumTrait, GetU64};

#[derive(Debug, Clone)]
pub struct VarInt(i32);
pub struct VarLong(i64);
pub struct Position(i32, i32, i16);
pub struct Angle(u8);
#[derive(Clone)]
pub struct UUID(u128);
pub struct BitSet(Vec<u64>);
pub struct FixedBitSet<const S: usize>([u8; S]); // INFO: S = ceil(size / 8)
#[derive(Clone)]
pub struct Array<T>(Vec<T>) where T: DataReader + DataWriter;
#[derive(Debug)]
pub struct Enum<T, S>(pub(crate) T, pub(crate) S) where T: ImportantEnumTrait, S: DataReader + GetU64;
pub struct ByteArray(Vec<u8>);
mod implementations;
mod important_functions;
