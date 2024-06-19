use binary_utils::{DataReader, DataWriter};

/// An Item inside a inventory window
pub struct Slot;
impl DataReader for Slot {
    async fn read(_reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        todo!()
    }
}
impl DataWriter for Slot {
    async fn write(&self, _writer: &mut (impl tokio::io::AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
