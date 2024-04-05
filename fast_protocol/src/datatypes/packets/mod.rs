use binary_utils::{DataReader as _, ListDataReader as _, Error, PacketReader, Result, DataWriter};
mod handshake;
mod login;
mod status;
mod configuration;
mod playing;
use datatypes::{ImportantFunctions as _, VarInt};
pub use handshake::*;
pub use login::*;
pub use status::*;
pub use configuration::*;
pub use playing::*;
use tokio::io::{AsyncWrite, BufReader, AsyncBufReadExt};
use tokio::io::AsyncReadExt as _;
use tokio::net::tcp::ReadHalf;
#[derive(Clone, Copy)]
pub enum State {
    Handshake,
    Login,
    Status,
    Configuration,
    Playing,
}
pub enum ClientboundPackets {
    LegacyPong(LegacyPongPacket),
    Pong(PongPacket),
    Status(StatusResponsePacket),
    LoginSuccess(LoginSuccess),
    LoginDisconnect(LoginDisconnect),
    LoginEncryptionRequest(LoginEncryptionRequest),
    ConfigurationPluginMessage(ConfigurationClientboundPluginMessage),
    RegistryData(RegistryData),
    RemoveResourcePack(RemoveResoucePack),
    AddResourcePack(AddResourcePack),
    FeatureFlags(FeatureFlags),
    FinishConfiguration(FinishConfiguration),
}
impl DataWriter for ClientboundPackets {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match self {
            Self::LegacyPong(packet) => packet.write(writer).await,
            Self::Pong(packet) => packet.write(writer).await,
            Self::LoginDisconnect(packet) => packet.write(writer).await,
            Self::LoginSuccess(packet) => packet.write(writer).await,
            Self::Status(packet) => packet.write(writer).await,
            Self::LoginEncryptionRequest(packet) => packet.write(writer).await,
            Self::ConfigurationPluginMessage(packet) => packet.write(writer).await,
            Self::RegistryData(packet) => packet.write(writer).await,
            Self::RemoveResourcePack(packet) => packet.write(writer).await,
            Self::AddResourcePack(packet) => packet.write(writer).await,
            Self::FeatureFlags(packet) => packet.write(writer).await,
            Self::FinishConfiguration(packet) => packet.write(writer).await,
        }
    }
}
pub enum ServerboundPackets {
    None,
    LegacyPing,
    Handshake(HandshakePacket),
    StatusRequest(StatusRequestPacket),
    PingRequest(PingPacket),
    LoginStart(LoginStart),
    LoginEncryptionResponse(LoginEncryptionResponse),
    LoginAcknowledged,
    ClientInformation(ClientInformation),
    ConfigurationPluginMessage(ServerboundPluginMessage),
    AckFinishConfiguration,
    KeepAlive(KeepAliveResponse),
    Pong,
    ResoucePackResponse(ResoucePackResponse),
}
impl ServerboundPackets {
    // pub async fn read(reader: &mut (impl AsyncRead + Unpin), state: State) -> Result<Self> {
    pub async fn read<'a>(reader: &mut BufReader<ReadHalf<'a>>, state: &State) -> Result<Self> {
        if let Ok(vec) = reader.fill_buf().await {
            println!("vec: {:?}", vec);
            if vec.is_empty() {
                return Ok(ServerboundPackets::None);
            }
        } else {
            return Err(Error::NotEnoughtBytes("".to_string()))
        }
        if reader.buffer().len() == 0 { return Ok(ServerboundPackets::None) }
        match state {
            State::Handshake => Self::handshake(reader).await,
            State::Status => Self::status(reader).await,
            State::Login => Self::login(reader).await,
            State::Configuration => Self::configuration(reader).await,
            State::Playing => Self::playing(reader).await,
        }
    }
    async fn handshake<'a>(reader: &mut BufReader<ReadHalf<'a>>) -> Result<Self> {
        // let mut reader = BufReader::new(reader);
        let mut first_two_bytes = bytes::BytesMut::with_capacity(2);
        match reader.read_buf(&mut first_two_bytes).await {
            Ok(_) => /*println!("Read to the buffer")*/(),
            Err(_) => return Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into(),
        }
        if first_two_bytes[0] == 0xFE && first_two_bytes[1] == 0x01 {
            LegacyPingPacket::read(reader).await?;
            Ok(Self::LegacyPing)
        } else {
            let length;
            let packet_id;
            if first_two_bytes[0] >= 0x80 {
                length = VarInt::new((((first_two_bytes[0] as u16) & 0x7F) | ((first_two_bytes[0] as u16) << 7)) as i32);
                packet_id = VarInt::read(reader).await?;
            } else {
                length = VarInt::new(first_two_bytes[0] as i32);
                packet_id = VarInt::new(first_two_bytes[1] as i32);
            }
            match packet_id.get_value() {
                0 => {
                    let data = HandshakePacket::read(reader, length.get_value(), packet_id.get_value()).await?;
                    Ok(ServerboundPackets::Handshake(data))
                },
                _ => Error::InvalidId.into()
            }
        }
    }
    async fn status<'a>(reader: &mut BufReader<ReadHalf<'a>>) -> Result<Self> {
        let length = VarInt::read(reader).await?.get_value();
        let packet_id = VarInt::read(reader).await?.get_value();
        match packet_id {
            0 => {
                Ok(ServerboundPackets::StatusRequest(StatusRequestPacket::read(reader, length, packet_id).await?))
            },
            1 => {
                Ok(ServerboundPackets::PingRequest(PingPacket::read(reader, length, packet_id).await?))
            },
            _ => Error::InvalidId.into(),
        }
    }
    async fn login<'a>(reader: &mut BufReader<ReadHalf<'a>>) -> Result<Self> {
        let _length = VarInt::read(reader).await?.get_value();
        let packet_id = VarInt::read(reader).await?.get_value();
        match packet_id {
            0 => Ok(Self::LoginStart(LoginStart::read(reader).await?)),
            1 => Ok(Self::LoginEncryptionResponse(LoginEncryptionResponse::read(reader).await?)),
            3 => Ok(Self::LoginAcknowledged),
            _ => todo!(),
        }
    }
    async fn configuration<'a>(reader: &mut BufReader<ReadHalf<'a>>) -> Result<Self> {
        reader.buffer();
        let length = VarInt::read(reader).await?.get_value();
        let packet_id = VarInt::read(reader).await?.get_value();
        match packet_id {
            0 => Ok(Self::ClientInformation(ClientInformation::read(reader).await?)),
            1 => Ok(Self::ConfigurationPluginMessage(ServerboundPluginMessage::read_list(reader, length as usize - 1).await?)),
            2 => Ok(Self::AckFinishConfiguration),
            3 => Ok(Self::KeepAlive(KeepAliveResponse::read(reader).await?)),
            4 => unimplemented!(),
            5 => Ok(Self::ResoucePackResponse(ResoucePackResponse::read(reader).await?)),
            i32::MIN..=-1 | 6..=i32::MAX => Err(Error::InvalidStructure),
        }
    }
    async fn playing<'a>(_reader: &mut BufReader<ReadHalf<'a>>) -> Result<Self> {
        todo!()
    }

}
