use serde::Serialize;

use crate::{datatypes::datatype_definition::{JSONTextComponent, VarInt, String, ByteArray, UUID}, utils::{DataWriter, write_bytes}};

pub struct LoginDisconnect(JSONTextComponent);
pub struct EncryptionRequest(String, VarInt, ByteArray, VarInt, ByteArray);
pub struct LoginSuccess(UUID, String, VarInt/*, TODO: Insert Array */);
#[derive(Serialize)]
struct Reason {
    reason: std::string::String,
}
impl LoginDisconnect {
    pub fn new(reason: std::string::String) -> Self {
        Self(JSONTextComponent::from(Reason { reason }))
    }
}
impl EncryptionRequest {
    pub fn new() -> Self {
        todo!()
    }
}
impl DataWriter for LoginDisconnect {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x00);
        id.write(&mut data)?;
        self.0.write(&mut data)?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
impl DataWriter for EncryptionRequest {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x01);
        id.write(&mut data)?;
        self.0.write(&mut data)?;
        self.1.write(&mut data)?;
        self.2.write(&mut data)?;
        self.3.write(&mut data)?;
        self.4.write(&mut data)?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
