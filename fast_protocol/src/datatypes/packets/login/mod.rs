use serde::Serialize;
use tokio::io::AsyncWrite;

use crate::utils::{DataWriter, write_bytes};
use crate::datatypes::datatype_definition::{JSONTextComponent, VarInt, String, ByteArray, UUID, Boolean, ImportantFunctions as _};

const SKIN: &str = "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmM5NmExNGRjMWNiOTQzYjhmZjNjOTJhYWNiMDEwMmMyMzg5ZWVkZWY1MGQzNmI3MTRkMGRiOThiMjdhIn19fQ";

pub struct LoginDisconnect(JSONTextComponent);
pub struct EncryptionRequest(String, VarInt, ByteArray, VarInt, ByteArray);
pub struct LoginDataStruct(String, String, Boolean, Option<String>);
pub struct LoginSuccess(UUID, String, Vec<LoginDataStruct>);
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
impl Default for LoginDataStruct {
    fn default() -> Self {
        Self(
            String::new(std::string::String::from("texture")),
            String::new(format!("data:image/png;base64,{}", SKIN)),
            Boolean::new(false),
            None,
        )
    }
}
impl LoginSuccess {
    pub fn new(uuid: UUID, username: String) -> Self {
        let prop = vec![LoginDataStruct::default()];
        Self(uuid, username, prop)
    }
}
impl DataWriter for LoginDisconnect {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x00);
        id.write(&mut data).await?;
        self.0.write(&mut data).await?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer).await?;
        write_bytes(writer, &data).await?;
        Ok(())
    }
}
impl DataWriter for EncryptionRequest {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x01);
        id.write(&mut data).await?;
        self.0.write(&mut data).await?;
        self.1.write(&mut data).await?;
        self.2.write(&mut data).await?;
        self.3.write(&mut data).await?;
        self.4.write(&mut data).await?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer).await?;
        write_bytes(writer, &data).await?;
        Ok(())
    }
}
impl DataWriter for LoginDataStruct {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        self.0.write(writer).await?;
        self.1.write(writer).await?;
        self.2.write(writer).await?;
        if let Some(data) = self.3.clone() {
            data.write(writer).await?;
        }
        Ok(())
    }
}
impl DataWriter for LoginSuccess {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        self.0.write(&mut data).await?;
        self.1.write(&mut data).await?;
        for entry in self.2.iter() {
            entry.write(&mut data).await?;
        }
        let length = VarInt::new(data.len() as i32);
        length.write(writer).await?;
        write_bytes(writer, &data).await?;
        Ok(())
    }
}
