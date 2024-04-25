use crate::{NbtValue, error::NbtResult, reader::NbtReader};

/// Trait for every type that has to be write NBT data
pub trait NbtWrite {
    /// function to write a signed 8-bit integer array
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]);
    /// function to write a signed 32-bit integer array
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]);
    /// function to write a signed 64-bit integer array
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]);
    /// function to write a string
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str);
    /// function to write a list of nbt values
    fn write_list(writer: &mut Vec<u8>, data: &[NbtValue]) -> NbtResult<()>;
    /// function to write a list of named nbt values
    fn write_compound(writer: &mut Vec<u8>, name: Option<&String>, data: &[(String, NbtValue)]) -> NbtResult<()>;
    /// function to write nbt data to an vector
    fn write_to(value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;
    /// function to write nbt data to an vector with an name
    fn write_to_with_name(name: &str, value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;
    /// function to write a text component to an vector
    fn write_text_component(writer: &mut Vec<u8>, value: &NbtValue) -> NbtResult<()>;
    /// function to convert nbt data into bytes
    fn to_bytes(value: &NbtValue) -> NbtResult<Vec<u8>> {
        let mut buff = Vec::new();
        Self::write_to(value, &mut buff)?;
        Ok(buff)
    }
}

/// trait for every type that has to be read as NBT data
pub trait NbtRead {
    /// function to read a signed 8-bit integer array
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>>;
    /// function to read a signed 32-bit integer array
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>>;
    /// function to read a signed 64-bit integer array
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>>;
    /// function to read a string
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String>;
    /// function to read a list of nbt values
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>>;
    /// function to read a list of named nbt values
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>>;
    /// function to read nbt data from a `NbtReader`
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn from_reader(reader: NbtReader) -> NbtResult<NbtValue>;
    /// function to read a text component from a `NbtReader`
    ///
    /// # Arguments
    /// `reader` - A mutable reference to `NbtReader`
    ///
    /// # Returns
    ///
    /// Returns a Result with the value of an Error, why the read failed
    fn from_reader_text_component(reader: NbtReader) -> NbtResult<NbtValue>;
}
/// A trait that allows structs to have a function that transrforms themself into a `NbtValue`
pub trait IntoNbt {
    /// Converts this struct into a `NbtValue`
    fn to_nbt(&self) -> NbtValue;
}
