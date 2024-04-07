use binary_utils::{DataWriter, write_bytes};
use nbt_lib::{NbtValue, version::JavaNetty, traits::NbtWrite};
use tokio::io::AsyncWrite;

pub mod important_enums;

pub struct EntityMetadata(entmet_lib::EntityMetadata);
impl DataWriter for EntityMetadata {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
