use binary_utils::{DataWriter, DataReader, write_bytes, Error};
use datatypes::ImportantFunctions as _;
use tokio::io::{AsyncWrite, AsyncRead, BufWriter, AsyncWriteExt};
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
        let mut buf_writer = BufWriter::new(writer);
        let mut data = Vec::new();
        let id = VarInt::new(0);
        id.write(&mut data).await?;
        self.channel.write(&mut data).await?;
        self.data.write(&mut data).await?;
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for ConfigurationDisconnect {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut buf_writer = BufWriter::new(writer);
        let mut data = Vec::new();
        let id = datatypes::VarInt::new(0x01);
        id.write(&mut data).await?;
        self.reason.write(&mut data).await?;
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for FinishConfiguration {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut buf_writer = BufWriter::new(writer);
        VarInt::new(1).write(&mut buf_writer).await?;
        VarInt::new(0x02).write(&mut buf_writer).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for KeepAlive {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut buf_writer = BufWriter::new(writer);
        let mut data = Vec::new();
        let id = VarInt::new(0x03);
        id.write(&mut data).await?;
        self.id.write(&mut data).await?;
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for RegestryData {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut buf_writer = BufWriter::new(writer);
        let id = VarInt::new(0x05);
        let mut data = Vec::new();
        id.write(&mut data).await?;
        self.regestry_codec.write(&mut data).await?;
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for RemoveResoucePack {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::{VarInt, Boolean};
        let mut buf_writer = BufWriter::new(writer);
        let id = VarInt::new(0x06);
        let has_uuid = Boolean::new(self.uuid.is_some());
        let mut data = Vec::new();
        id.write(&mut data).await?;
        has_uuid.write(&mut data).await?;
        if let Some(uuid) = self.uuid.as_ref() {
            uuid.write(&mut data).await?;
        }
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(Error::FailedToWrite); }
        Ok(())
    }
}
impl DataWriter for AddResourcePack {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::{VarInt, Boolean};
        let mut buf_writer = BufWriter::new(writer);
        let id = VarInt::new(0x07);
        let forced = Boolean::new(self.forced);
        let has_prompt_message = Boolean::new(self.has_prompt_message.is_some());
        let mut data = Vec::new();
        id.write(&mut data).await?;
        self.uuid.write(&mut data).await?;
        self.url.write(&mut data).await?;
        self.hash.write(&mut data).await?;
        forced.write(&mut data).await?;
        has_prompt_message.write(&mut data).await?;
        if let Some(prompt_message) = self.has_prompt_message.as_ref() {
            prompt_message.write(&mut data).await?;
        }
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await { return Err(binary_utils::Error::FailedToWrite) }
        Ok(())
    }
}
impl DataWriter for FeatureFlags {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        use datatypes::VarInt;
        let mut buf_writer = BufWriter::new(writer);
        let mut data = Vec::new();
        VarInt::new(0x08).write(&mut data).await?;
        self.feature_count.write(&mut data).await?;
        self.feature_flags.write(&mut data).await?;
        VarInt::new(data.len() as i32).write(&mut buf_writer).await?;
        write_bytes(&mut buf_writer, &data).await?;
        if let Err(_) = buf_writer.flush().await {
            return Err(binary_utils::Error::FailedToWrite);
        }
        Ok(())
    }
}
