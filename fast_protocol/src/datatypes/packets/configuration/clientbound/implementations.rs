use binary_utils::{DataWriter, DataReader, write_bytes};
use datatypes::ImportantFunctions as _;
use tokio::io::{AsyncWrite, AsyncRead};

use crate::datatypes::packets::KeepAliveResponse;

use super::*;

impl DataWriter for TagArrayData {
    async fn write(&self, _writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        todo!()
    }
}
impl DataReader for TagArrayData {
    async fn read(_reader: &mut (impl AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        todo!()
    }
}
impl DataWriter for ClientboundPluginMessage {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut data = Vec::new();
        let id = VarInt::new(0);
        id.write(&mut data).await?;
        self.channel.write(&mut data).await?;
        self.data.write(&mut data).await?;
        let len = VarInt::new(data.len() as i32);
        let mut d = Vec::new();
        len.write(&mut d).await?;
        write_bytes(&mut d, &data).await?;
        write_bytes(writer, &d).await?;
        Ok(())
    }
}
impl DataWriter for ConfigurationDisconnect {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut data = Vec::new();
        let id = datatypes::VarInt::new(0x01);
        id.write(&mut data).await?;
        self.reason.write(&mut data).await?;
        let mut d = Vec::new();
        let len = datatypes::VarInt::new(data.len() as i32);
        len.write(&mut d).await?;
        write_bytes(&mut d, &data).await?;
        write_bytes(writer, &d).await
    }
}
impl DataWriter for FinishConfiguration {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let id = VarInt::new(0x02);
        let len = VarInt::new(1);
        let mut d = Vec::new();
        len.write(&mut d).await?;
        id.write(&mut d).await?;
        write_bytes(writer, &d).await
    }
}
impl DataWriter for KeepAlive {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut data = Vec::new();
        let id = VarInt::new(0x03);
        id.write(&mut data).await?;
        self.id.write(&mut data).await?;
        let len = VarInt::new(data.len() as i32);
        let mut d = Vec::new();
        len.write(&mut d).await?;
        write_bytes(&mut d, &data).await?;
        write_bytes(writer, &d).await
    }
}
