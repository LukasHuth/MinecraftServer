use binary_utils::DataWriter;
use nbt_lib::{NbtValue, version::JavaNetty};
use tokio::io::AsyncWrite;

pub mod important_enums;

// TODO replace fastnbt with own nbt value

pub struct EntityMetadata(entmet_lib::EntityMetadata);
pub struct Slot(slot_lib::Slot);
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
        todo!()
    }
}
