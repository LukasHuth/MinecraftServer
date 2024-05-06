//! A module for Datastructs that are important and need Nbtvalue's
use binary_utils::{DataWriter, write_bytes, DataReader};
use datatypes::ImportantFunctions;
use tokio::io::{AsyncWrite, AsyncRead, BufReader};

use crate::{NbtValue, version::JavaNetty, traits::{NbtWrite, NbtRead}, reader::NbtReader};
/// struct to hold and text component
///
/// # Note
///
/// This can be a `Compound` or a `String`
#[derive(Clone, Debug, PartialEq)]
pub struct TextComponent(NbtValue);
impl Default for TextComponent {
    fn default() -> Self {
        Self(NbtValue::Compound(
            Some("".to_string()),
            vec![("text".to_string(), NbtValue::String("".to_string()))]
                .into_iter().collect()
        ))
    }
}
/// Struct to hold a `NbtValue`
pub struct NBT(NbtValue);
impl From<String> for TextComponent{
    fn from(value: String) -> Self {
        Self (NbtValue::String(value))
    }
}
impl From<NbtValue> for NBT {
    fn from(value: NbtValue) -> Self {
        Self(value)
    }
}
impl DataWriter for TextComponent {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut data: Vec<u8> = Vec::new();
        if let Err(_) = JavaNetty::write_text_component(&mut data, &self.0) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        write_bytes(writer, &data).await
    }
}
impl DataReader for TextComponent {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        let buf_reader = BufReader::new(reader);
        let reader = NbtReader::new(buf_reader.buffer().to_vec());
        let value = JavaNetty::from_reader_text_component(reader);
        let value = match value {
            Ok(v) => v,
            Err(_) => return Err(binary_utils::Error::InvalidStructure),
        };
        Ok(Self(value))
    }
}
impl DataWriter for NBT {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut data: Vec<u8> = Vec::new();
        if let Err(_) = JavaNetty::write_to(&self.0, &mut data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        write_bytes(writer, &data).await
    }
}
impl DataReader for NBT {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        let buf_reader = BufReader::new(reader);
        let reader = NbtReader::new(buf_reader.buffer().to_vec());
        let value = JavaNetty::from_reader(reader);
        let value = match value {
            Ok(v) => v,
            Err(_) => return Err(binary_utils::Error::InvalidStructure),
        };
        Ok(Self(value))
    }
}
impl ImportantFunctions for NBT {
    type InputType = NbtValue;

    type ReturnType = NbtValue;

    fn new(data: Self::InputType) -> Self {
        Self(data)
    }

    fn get_value(&self) -> Self::ReturnType {
        self.0.clone()
    }
}
