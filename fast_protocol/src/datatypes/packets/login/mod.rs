use binary_utils::{DataWriter, Result, write_bytes, DataReader, ListDataReader as _};
use datatypes::{JSONTextComponent, VarInt, String, ByteArray, Boolean, UUID, ImportantFunctions as _};
use openssl::pkey::Private;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncWrite;

/// Static string representation of the default skin, that every user gets assigned who doesn't has
/// one.
///
/// # Note
///
/// Also used for all players, if the server is operating in offline mode
const SKIN: &str = "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmM5NmExNGRjMWNiOTQzYjhmZjNjOTJhYWNiMDEwMmMyMzg5ZWVkZWY1MGQzNmI3MTRkMGRiOThiMjdhIn19fQ";

// Clientbound
/// Packet to disconnect a client during `Login`
pub struct LoginDisconnect(JSONTextComponent);
/// Packet to start the encrypted authentication
pub struct LoginEncryptionRequest(String, VarInt, ByteArray, VarInt, ByteArray);
/// Data struct, used to represent data, that gets send to the client if the login is successful
pub struct LoginDataStruct(String, String, Boolean, Option<String>);
/// Packet to tell the client that the login was successful
pub struct LoginSuccess(UUID, String, Vec<LoginDataStruct>);
// serverbound
/// Packet used to start the login process
pub struct LoginStart {
    /// username of the player
    pub name: String,
    /// uuid of the player
    pub uuid: UUID,
}
/// Packet used to authenticate with the server
pub struct LoginEncryptionResponse {
    /// length of the shared secret
    pub shared_secret_length: VarInt,
    /// the value of the shared secret, encrypted with the server's public key
    pub shared_secret: ByteArray,
    /// length of the verify token
    pub verify_token_length: VarInt,
    /// verify token, encrypted with the same pulic key as the shared secret
    pub verify_token: ByteArray,
}
// Data Struct
/// Part of the session response this is data recieved by the mojang authetication service and
/// defined to be able to parse the data
#[derive(Deserialize)]
pub struct SessionResponsePiece {
    /// name of the data
    pub name: std::string::String,
    /// value of the data
    pub value: std::string::String,
    /// A signed signature
    pub signature: std::string::String,
}
/// Data struct holding the whole data response of the authetication service
#[derive(Deserialize)]
pub struct SessionResponse {
    /// profile id
    pub id: std::string::String,
    /// username of the player
    pub name: std::string::String,
    /// important properties send by the authetication service
    pub properties: Vec<SessionResponsePiece>,
}
#[derive(Serialize)]
struct Reason {
    reason: std::string::String,
}
impl LoginDisconnect {
    /// function to initialize a new instance of `LoginDisconnect`
    ///
    /// # Arguments
    /// `reason` - A reason, why the server closes the connection
    pub fn new(reason: std::string::String) -> Self {
        Self(JSONTextComponent::from(Reason { reason }))
    }
}
impl LoginEncryptionRequest {
    /// function to initialize a new instance of `LoginEncryptionRequest`
    ///
    /// # Arguments
    /// `key` - a rsa private key of the server used for the authentication precedure
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
    /// function to initialize a new instance of `LoginSuccess`
    ///
    /// # Arguments
    /// `uuid` - UUID of the connected player
    /// `username` - Username of the connected player
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
