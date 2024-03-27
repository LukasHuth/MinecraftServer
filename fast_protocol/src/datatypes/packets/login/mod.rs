use binary_utils::{DataWriter, Result, write_bytes, DataReader, ListDataReader as _};
use datatypes::{JSONTextComponent, VarInt, String, ByteArray, Boolean, UUID, ImportantFunctions as _};
use openssl::pkey::Private;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncWrite;

const SKIN: &str = "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmM5NmExNGRjMWNiOTQzYjhmZjNjOTJhYWNiMDEwMmMyMzg5ZWVkZWY1MGQzNmI3MTRkMGRiOThiMjdhIn19fQ";

// Clientbound
pub struct LoginDisconnect(JSONTextComponent);
pub struct LoginEncryptionRequest(String, VarInt, ByteArray, VarInt, ByteArray);
pub struct LoginDataStruct(String, String, Boolean, Option<String>);
pub struct LoginSuccess(UUID, String, Vec<LoginDataStruct>);
// serverbound
pub struct LoginStart {
    pub name: String,
    pub uuid: UUID,
}
pub struct LoginEncryptionResponse {
    pub shared_secret_length: VarInt,
    pub shared_secret: ByteArray,
    pub verify_token_length: VarInt,
    pub verify_token: ByteArray,
}
// Data Struct
#[derive(Deserialize)]
pub struct SessionResponsePiece {
    pub name: std::string::String,
    pub value: std::string::String,
    pub signature: std::string::String,
}
#[derive(Deserialize)]
pub struct SessionResponse {
    id: std::string::String,
    name: std::string::String,
    properties: Vec<SessionResponsePiece>,
}
#[derive(Serialize)]
struct Reason {
    reason: std::string::String,
}
impl LoginDisconnect {
    pub fn new(reason: std::string::String) -> Self {
        Self(JSONTextComponent::from(Reason { reason }))
    }
}
impl LoginEncryptionRequest {
    pub fn new(key: openssl::rsa::Rsa<Private>, verify_token: [u8;4]) -> Self {
        // let key = openssl::rsa::Rsa::generate(1024).unwrap();
        let key_bytes = key.public_key_to_der().unwrap();
        let server_id = String::new(std::string::String::from(""));
        let public_key_length = VarInt::new(key_bytes.len() as i32);
        let public_key = ByteArray::new(key_bytes);
        let verify_token_length = VarInt::new(4);
        let verify_token = ByteArray::new(verify_token.to_vec());
        Self(server_id, public_key_length, public_key, verify_token_length, verify_token)
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
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
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
impl DataWriter for LoginEncryptionRequest {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
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
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        let mut d = Vec::new();
        self.0.write(&mut d).await?;
        println!("d: {:?}", d);
        self.1.write(&mut d).await?;
        println!("d: {:?}", d);
        println!("{:?}", self.1);
        self.2.write(&mut d).await?;
        if let Some(data) = self.3.clone() {
            data.write(&mut d).await?;
        }
        write_bytes(writer, &d).await?;
        Ok(())
    }
}
impl DataWriter for LoginSuccess {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        println!("writing Success");
        let mut data = Vec::new();
        let packet_id = VarInt::new(0x02);
        packet_id.write(&mut data).await?;
        self.0.write(&mut data).await?;
        println!("{:?}", data);
        self.1.write(&mut data).await?;
        println!("{:?}", data);
        let amount = VarInt::new(self.2.len() as i32);
        amount.write(&mut data).await?;
        println!("{:?}", data);
        for entry in self.2.iter() {
            entry.write(&mut data).await?;
        }
        let length = VarInt::new(data.len() as i32);
        let mut d = Vec::new();
        length.write(&mut d).await?;
        write_bytes(&mut d, &data).await?;
        println!("{:?}", d);
        write_bytes(writer, &d).await?;
        Ok(())
    }
}
impl DataReader for LoginStart {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> Result<Self> {
        let name = String::read(reader).await?;
        let uuid = UUID::read(reader).await?;
        Ok(Self { name, uuid })
    }
}
impl DataReader for LoginEncryptionResponse {
    async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> Result<Self> {
        let shared_secret_length = VarInt::read(reader).await?;
        let shared_secret = ByteArray::read_list(reader, shared_secret_length.get_value() as usize).await?;
        let verify_token_length = VarInt::read(reader).await?;
        let verify_token = ByteArray::read_list(reader, verify_token_length.get_value() as usize).await?;
        Ok(Self{ shared_secret_length, shared_secret, verify_token_length, verify_token })
    }
}
