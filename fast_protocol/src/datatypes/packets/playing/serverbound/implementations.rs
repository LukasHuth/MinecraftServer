use binary_utils::{DataReader, ListDataReader};
use datatypes::{
    Array, BitMask, Boolean, Byte, Double, Enum, FixedBitSet, FixedByteArray, Float, Identifier, ImportantFunctions, Long, UnsignedByte, VarInt
};

impl DataReader for super::ConfirmTeleportation {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            t_id: VarInt::read(reader).await?,
        })
    }
}
impl DataReader for super::ChangeDifficulty {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            new_difficulty: Enum::read(reader).await?,
        })
    }
}
impl DataReader for super::AcknowledgeMessage {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            message_count: VarInt::read(reader).await?,
        })
    }
}
impl DataReader for super::ChatCommand {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            command: datatypes::String::read(reader).await?,
        })
    }
}
impl DataReader for super::SignedChatCommand {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        let command = datatypes::String::read(reader).await?;
        let timestamp = Long::read(reader).await?;
        let salt = Long::read(reader).await?;
        let array_length = VarInt::read(reader).await?;
        let length = array_length.get_value() as usize;
        Ok(Self {
            command,
            timestamp,
            salt,
            array_length,
            arrray_of_argument_signatures: Array::read_list(reader, length).await?,
            message_count: VarInt::read(reader).await?,
            acknowledged: FixedBitSet::read(reader).await?,
        })
    }
}
impl DataReader for super::ChatMessage {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        let message = datatypes::String::read(reader).await?;
        let timestamp = Long::read(reader).await?;
        let salt = Long::read(reader).await?;
        let has_signature = Boolean::read(reader).await?;
        let signature = if has_signature.get_value() {
            Some(FixedByteArray::read(reader).await?)
        } else {
            None
        };
        Ok(Self {
            message,
            timestamp,
            salt,
            has_signature,
            signature,
            message_count: VarInt::read(reader).await?,
            acknowledged: FixedBitSet::read(reader).await?,
        })
    }
}
impl DataReader for super::ChunkBatchReceived {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            chunks_per_tick: Float::read(reader).await?,
        })
    }
}
impl DataReader for super::ClientStatus {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            action: Enum::read(reader).await?,
        })
    }
}
impl DataReader for super::ClientInformation {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            locale: datatypes::String::read(reader).await?,
            view_distance: Byte::read(reader).await?,
            chat_mode: Enum::read(reader).await?,
            chat_colors: Boolean::read(reader).await?,
            displayed_skin_parts: BitMask::read(reader).await?,
            main_hand: Enum::read(reader).await?,
            enable_text_filtering: Boolean::read(reader).await?,
            allow_server_listings: Boolean::read(reader).await?,
        })
    }
}
impl DataReader for super::CommandSuggestionRequest {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            transaction_id: VarInt::read(reader).await?,
            text: datatypes::String::read(reader).await?,
        })
    }
}
impl DataReader for super::AcknowledgeConfiguration {
    async fn read(_reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {})
    }
}
impl DataReader for super::CloseContainer {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            window_id: UnsignedByte::read(reader).await?,
        })
    }
}
impl DataReader for super::PluginMessage {
    async fn read(_reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        todo!()
    }
}
impl DataReader for super::KeepAlive {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            id: Long::read(reader).await?,
        })
    }
}
impl DataReader for super::LockDifficulty {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            locket: Boolean::read(reader).await?,
        })
    }
}
impl DataReader for super::SetPlayerPosition {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        Ok(Self {
            x: Double::read(reader).await?,
            feet_y: Double::read(reader).await?,
            z: Double::read(reader).await?,
            on_ground: Boolean::read(reader).await?,
        })
    }
}
