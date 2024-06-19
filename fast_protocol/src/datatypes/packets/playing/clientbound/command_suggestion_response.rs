use datatypes::{Array, VarInt};

/// Module to extend the `CommandSuggestionResponse`
pub mod command_suggestion_response_data {
    use binary_utils::{DataReader, DataWriter};
    use datatypes::{Boolean, ImportantFunctions as _, String};
    use nbt_lib::datatypes::TextComponent;

    /// a possible value to insert
    pub struct CommandSuggestionResponseData {
        /// The value to insert
        pub value: String,
        /// Optional tooltip to explain the suggested value
        pub tooltip: Option<TextComponent>,
    }
    impl DataWriter for CommandSuggestionResponseData {
        async fn write(&self, writer: &mut (impl tokio::io::AsyncWrite + Unpin)) -> binary_utils::Result<()> {
            self.value.write(writer).await?;
            Boolean::new(self.tooltip.is_some()).write(writer).await?;
            if let Some(ref tooltip) = self.tooltip {
                tooltip.write(writer).await?;
            }
            Ok(())
        }
    }
    impl DataReader for CommandSuggestionResponseData {
        async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
            let value = String::read(reader).await?;
            let has_tooltip: bool = Boolean::read(reader).await?.get_value();
            let tooltip = if has_tooltip {
                Some(TextComponent::read(reader).await?)
            } else { None };
            Ok(Self {
                value,
                tooltip,
            })
        }
    }
}
/// A Response for the auto completion of commands
pub struct CommandSuggestionResponse {
    /// Transaction ID
    pub id: VarInt,
    /// The start of the text to replace
    pub start: VarInt,
    /// The length of the text to replace
    pub length: VarInt,
    /// A list of all options
    pub matches: Array<command_suggestion_response_data::CommandSuggestionResponseData>
}
