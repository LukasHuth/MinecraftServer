use binary_utils::{DataWriter, write_bytes};
use datatypes::ImportantFunctions;
use tokio::io::AsyncWrite;

use crate::{NbtValue, version::JavaNetty, traits::NbtWrite as _};
#[derive(Clone)]
pub struct TextComponent(NbtValue);
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
impl DataWriter for NBT {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut data: Vec<u8> = Vec::new();
        if let Err(_) = JavaNetty::write_to(&self.0, &mut data) {
            return Err(binary_utils::Error::FailedToWrite);
        }
        write_bytes(writer, &data).await
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
