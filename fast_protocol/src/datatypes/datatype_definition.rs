use binary_utils::DataWriter;
use tokio::io::AsyncWrite;

/// Module containing important enums needed by the packets
pub mod important_enums;

/// Packet containing entity metadata
///
/// # Note
///
/// This struct will probably move into the entity metadata library when it is finished
pub struct EntityMetadata(/*entmet_lib::EntityMetadata*/);
impl DataWriter for EntityMetadata {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
