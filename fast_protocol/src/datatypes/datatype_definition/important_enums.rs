use binary_utils::Error;
use datatypes::ImportantEnumTrait;

/// Enum used to be able to select `Status` or `Login` in the `Handshake` packet
#[derive(Debug, Clone)]
pub enum HandshakeNextState {
    /// This option selects to switch to `Status`
    Status,
    /// This option selects to switch to `Login`
    Login,
}
impl ImportantEnumTrait for HandshakeNextState {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            1 => Ok(HandshakeNextState::Status),
            2 => Ok(HandshakeNextState::Login),
            _ => Error::InvalidStructure.into()
        }
    }
}
