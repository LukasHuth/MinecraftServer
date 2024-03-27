use binary_utils::Error;
use datatypes::ImportantEnumTrait;

#[derive(Debug, Clone)]
pub enum HandshakeNextState {
    Status,
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
