use binary_utils::{DataReader, Result, ListDataReader, DataWriter};
use tokio::io::AsyncRead;

use super::*;

impl DataReader for ClientInformation {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let locale = datatypes::String::read(reader).await?;
        let view_distance = datatypes::Byte::read(reader).await?;
        let chat_mode = datatypes::Enum::read(reader).await?;
        let chat_colors = datatypes::Boolean::read(reader).await?;
        let displayed_skin_parts = datatypes::UnsignedByte::read(reader).await?;
        let main_hand = datatypes::Enum::read(reader).await?;
        let text_filtering = datatypes::Boolean::read(reader).await?;
        let allow_server_listing = datatypes::Boolean::read(reader).await?;
        Ok(Self {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
            text_filtering,
            allow_server_listing
        })
    }
}
impl ListDataReader for ServerboundPluginMessage {
    async fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> Result<Self> {
        let channel = datatypes::Identifier::read(reader).await?;
        let mut data = Vec::new();
        channel.write(&mut data).await?;
        let data = datatypes::ByteArray::read_list(reader, length - data.len()).await?;
        Ok(Self { channel, data })
    }
}
impl DataReader for KeepAliveResponse {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let id = datatypes::Long::read(reader).await?;
        Ok(Self { id } )
    }
}
impl DataReader for ResoucePackResponse {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let uuid = datatypes::UUID::read(reader).await?;
        let result = datatypes::Enum::read(reader).await?;
        Ok(Self { uuid, result } )
    }
}
