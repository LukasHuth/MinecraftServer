use binary_utils::{DataWriter, Result, DataReader, read_utf8_char, write_bytes};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncRead};
use super::*;
use crate::ImportantFunctions;

impl DataWriter for Identifier {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        self.0.write(writer).await
    }
}
impl DataReader for Identifier {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let data = String::read(reader).await?;
        Ok(Self(data))
    }
}
impl<T> From<T> for JSONTextComponent where T: Serialize {
    fn from(value: T) -> Self {
        Self(String::new(serde_json::to_string(&value).unwrap_or(std::string::String::new())))
    }
}
impl DataWriter for JSONTextComponent {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        self.0.write(writer).await?;
        Ok(())
    }
}
impl DataReader for String {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        use crate::VarInt;
        let length = VarInt::read(reader).await?;
        let mut chars = Vec::new();
        for _ in 0..length.get_value() {
            chars.push(read_utf8_char(reader, line!(), file!()).await?);
        }
        let data: std::string::String = chars.iter().collect();
        Ok(Self(data))
    }
}
impl DataWriter for String {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        use crate::VarInt;
        let bytes = self.0.as_bytes();
        let length = VarInt::new(bytes.len() as i32);
        length.write(writer).await?;
        write_bytes(writer, bytes).await
    }
}
