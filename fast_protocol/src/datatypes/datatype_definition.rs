use binary_utils::{DataWriter, write_bytes};
use nbt_lib::{NbtValue, version::JavaNetty, traits::NbtWrite};
use tokio::io::AsyncWrite;

pub mod important_enums;

pub struct EntityMetadata(entmet_lib::EntityMetadata);
pub struct Slot(slot_lib::Slot);
#[derive(Clone)]
pub struct TextComponent(NbtValue);
pub struct NBT(NbtValue);

impl From<String> for TextComponent{
    fn from(value: String) -> Self {
        Self (NbtValue::String(value))
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
impl DataWriter for Slot {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
impl DataWriter for EntityMetadata {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
