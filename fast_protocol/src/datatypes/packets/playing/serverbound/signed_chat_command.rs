use datatypes::{Array, FixedBitSet, Long, VarInt};

/// Signed Chat Command
#[allow(missing_docs)]
pub struct SignedChatCommand {
    pub command: datatypes::String,
    pub timestamp: Long,
    pub salt: Long,
    pub array_length: VarInt,
    pub arrray_of_argument_signatures: Array<signed_chat_command_extra::ArgumentSignature>,
    pub message_count: VarInt,
    pub acknowledged: FixedBitSet<20>,
}
#[allow(missing_docs)]
pub mod signed_chat_command_extra {
    use binary_utils::{DataReader, DataWriter};
    use datatypes::FixedByteArray;

    pub struct ArgumentSignature {
        pub argument_name: datatypes::String,
        pub signature: FixedByteArray<256>,
    }
    impl DataWriter for ArgumentSignature {
        async fn write(
            &self,
            _writer: &mut (impl tokio::io::AsyncWrite + Unpin),
        ) -> binary_utils::Result<()> {
            unreachable!()
        }
    }
    impl DataReader for ArgumentSignature {
        async fn read(
            reader: &mut (impl tokio::io::AsyncRead + Unpin),
        ) -> binary_utils::Result<Self> {
            Ok(Self {
                argument_name: datatypes::String::read(reader).await?,
                signature: FixedByteArray::read(reader).await?,
            })
        }
    }
}
