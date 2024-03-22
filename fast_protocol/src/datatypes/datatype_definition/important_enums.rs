use crate::{ImportantEnumTrait, errors::Error};

#[derive(Debug, Clone)]
pub enum HandshakeNextState {
    Status,
    Login,
}
impl ImportantEnumTrait for HandshakeNextState {
    fn new(data: u64) -> crate::errors::Result<Self> {
        match data {
            1 => Ok(HandshakeNextState::Status),
            2 => Ok(HandshakeNextState::Login),
            _ => Error::InvalidStructure.into()
        }
    }
}
