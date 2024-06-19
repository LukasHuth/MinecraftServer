#![deny(missing_docs)]
//! This is a crate that defines neccessary structs and functions to handle slots in Minecraft

use nbt_lib::NbtValue;

/// This Module defines Datatypes associated with this library
pub mod datatypes;
/// Enum of a slot
///
/// This enum can either be `Empty` or hold data of an inventory slot
#[derive(PartialEq)]
pub enum Slot {
    /// This variant tells, that the slot is empty
    Empty,
    /// This variant holds slot data
    ///
    /// The data recorded is this:
    /// - Item id
    /// - Item amount
    /// - Optional NBT data of the item in the slot
    // Data(::datatypes::VarInt,::datatypes::Byte, Option<nbt_lib::datatypes::NBT>)
    Data(i32,i8, Option<NbtValue>)
}
impl Default for Slot {
    fn default() -> Self {
        Self::Empty
    }
}
/// The new Slot data
pub mod new;
