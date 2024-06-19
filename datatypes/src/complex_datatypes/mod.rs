use binary_utils::{DataReader, DataWriter};

use crate::{ImportantEnumTrait, GetU64};

#[derive(Debug, Clone)]
/// A wrapper struct for a signed 32-bit value, typically used for implementing `DataReader` and `DataWriter`.
/// But the writen size of this type can vary based on the size of the data.
///
/// Variable-length format such that smaller numbers use fewer bytes. The 7 least significant bits are used to 
/// encode the value and the most significant bit indicates whether there's another byte after it for the next
/// part of the number. The least significant group is written first, followed by each of the more significant
/// groups; thus, VarInts are effectively little endian (however, groups are 7 bits, not 8).
///
/// VarInts are never longer than 5 bytes
pub struct VarInt(i32);
/// A wrapper struct for a signed 64-bit value, typically used for implementing `DataReader` and `DataWriter`.
/// But the writen size of this type can vary based on the size of the data.
///
/// Variable-length format such that smaller numbers use fewer bytes. The 7 least significant bits are used to 
/// encode the value and the most significant bit indicates whether there's another byte after it for the next
/// part of the number. The least significant group is written first, followed by each of the more significant
/// groups; thus, VarLongs are effectively little endian (however, groups are 7 bits, not 8).
///
/// VarLongs are never longer than 10 bytes
pub struct VarLong(i64);
/// A wraper struct to hold `x`, `z` and `y` where `y` is the height
///
/// # Note
///
/// An integer/block position consists of three components:
///
/// - `x` ranging from -33554432 to 33554431.
/// - `z` ranging from -33554432 to 33554431.
/// - `y` ranging from -2048 to 2047.pub struct Position(i32, i32, i16);
#[derive(PartialEq, Debug)]
pub struct Position(i32, i32, i16);
/// A wrapper struct representing a rotation angle in steps of 1/256 of a full turn.
///
/// The `Angle` struct provides a convenient way to work with rotation angles, where each step
/// represents 1/256 of a full turn. This allows for precise control over rotation measurements.
///
/// # Note
///
/// The range of valid values for an `Angle` instance is from 0 to 255, representing a full
/// rotation (0 degrees to 360 degrees) in steps of 1/256 of a full turn.
///
pub struct Angle(u8);
/// A wrapper holding an unsigned 128-bit integer representing an UUID
#[derive(Clone)]
pub struct UUID(u128);
/// A wrapper for a bitset, containing n/64 values where n is the amount of bits
pub struct BitSet(Vec<u64>);
/// A wrapper for a bitset, containing n/64 values where n is the amount of bits
/// But the size n = S is defined from the beginning
pub struct FixedBitSet<const S: usize>([u8; S]); // INFO: S = ceil(size / 8)
/// A wrapper containing a list of type `T` that implements `DataReader` and `DataWriter`
#[derive(Clone)]
pub struct Array<T>(Vec<T>) where T: DataReader + DataWriter;
/// A wrapper containing an enum `T`, represented by the type `S`
/// T needs to implement `ImportantEnumTrait`
/// S needs to implement `DataReader` + `GetU64`
#[derive(Debug)]
pub struct Enum<T, S>(pub(crate) T, pub(crate) S) where T: ImportantEnumTrait, S: DataReader + GetU64;
/// Fixed point data
pub struct FixedPoint<T, const S: u64>(T) where T: GetU64;
/// A wrapper containing a List of unsigned 8-bit integers
pub struct ByteArray(Vec<u8>);
mod implementations;
mod important_functions;
mod special_functions;
