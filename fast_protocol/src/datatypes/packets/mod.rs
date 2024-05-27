use binary_utils::{DataReader as _, ListDataReader as _, Error, PacketReader, Result, DataWriter};
pub mod handshake;
pub mod login;
pub mod status;
pub mod configuration;
pub mod playing;
use datatypes::{ImportantFunctions as _, VarInt};
use handshake::*;
use login::*;
use status::*;
use configuration::{clientbound::*, serverbound::*};
use playing::{clientbound::*, serverbound::*};
use tokio::io::{AsyncWrite, BufReader, AsyncBufReadExt};
use tokio::io::AsyncReadExt as _;
use tokio::net::tcp::ReadHalf;
/// Enum of all possible states that a connection can have while being established
#[derive(Clone, Copy)]
pub enum State {
    /// The initial handshake used to decide if `Status` or `Login` should be the next State
    Handshake,
    /// State used for Authentication, name specification and etc.
    Login,
    /// Used to get the motd or to know the ping
    Status,
    /// State between `Login` and `Playing`
    /// Used to configure basic informations and variables important for the client to play on the
    /// server
    Configuration,
    /// State the player is in most of the time, this state is active after `Configuration` and
    /// is active till the player leaves the Server.
    Playing,
}
/// An enum capable of holding every Clientbound Packet (Server to Client) to be able to store them
/// Effectively.
pub enum ClientboundPackets {
    /// Legacy Ping Response to notify Minecraft versions < 1.7 that the server is older than them
    /// and shows the player count, max players, motd and the ping.
    LegacyPong(LegacyPongPacket),
    /// The Ping Response to a Ping Request of the Current Protocol
    Pong(PongPacket),
    /// The Status Response to a status request of the current protocol
    Status(StatusResponsePacket),
    /// Packet to notify the Client that the authentication was successful and the state moves to
    /// `Configuration`
    LoginSuccess(LoginSuccess),
    /// Packet to notify the Client that the authentication was not successful and that the
    /// connection gets terminated
    LoginDisconnect(LoginDisconnect),
    /// Packet to Request an encrypted authentication
    ///
    /// # Note
    /// This Packet only gets used on servers with enabled onlinemode
    LoginEncryptionRequest(LoginEncryptionRequest),
    /// Packet for Client server communication, mostly used for mods
    ConfigurationPluginMessage(ConfigurationClientboundPluginMessage),
    /// This Packet can be used to customize registries of the Client to be able to customize the
    /// game in a better way.
    RegistryData(RegistryData),
    /// Packet to Remove a certain Texture Pack
    ///
    /// # Note
    ///
    /// This packet is also capable to request to remove all Texturepacks
    RemoveResourcePack(RemoveResoucePack),
    /// Packet to let the user add an Texturepack
    ///
    /// # Note
    ///
    /// These Texturepacks can also be forced. If the client doesn't apply those, the client should
    /// be disconnected from the server
    AddResourcePack(AddResourcePack),
    /// Packet to enable certain features, like balances introduced in later versions of the game
    ///
    /// List of the available feature flags as of version 1.20.4
    /// - minecraft:vanilla: Tells the client that the server is vanilla without mods.
    /// - minecraft:bundle: Enables bundles on the client
    /// - minecraft:trade_rebalance: enables the support for the rebalanced villager trades
    /// - minecraft:update_1_21: enables support for 1.21 features
    FeatureFlags(FeatureFlags),
    /// Packet to tell the client that the configuration is finished and should switch the state to
    /// `Playing`
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
/// A list of all Serverbound Packets that can be received (Client to Server)
pub enum ServerboundPackets {
    /// Variant when no packet was received
    None,
    /// Information that a legacy ping was received, no information stored, because none is needed
    LegacyPing,
    /// Handshake initialization Packet
    ///
    /// # Note
    ///
    /// Every Connection that is not a legacy connection has to start with this packet to tell the
    /// server if the connection is of type `Status` or `Login`
    Handshake(HandshakePacket),
    /// Packet to request the player count, max players and motd
    StatusRequest(StatusRequestPacket),
    /// Packet to measure the Ping, should be responded with the same id
    PingRequest(PingPacket),
    /// Packet to start the login procedure
    LoginStart(LoginStart),
    /// Packet that sends encrypted authentication data for the server to autheticate the minecraft
    /// account with the official authentication service
    LoginEncryptionResponse(LoginEncryptionResponse),
    /// Packet that assures the server that the `LoginSuccess` was received
    LoginAcknowledged,
    /// First packet sent to the server after switching to `Configuration`
    ClientInformation(ClientInformation),
    /// Plugin message packet for the configuration phase
    ConfigurationPluginMessage(ServerboundPluginMessage),
    /// Packet that assures the server that the `FinishConfiguration` Packet was received
    AckFinishConfiguration,
    /// Keep Alive Packet, needed to be received to not get disconnected
    KeepAlive(KeepAliveResponse),
    /// Answer to a ping packet requested in the `Configuration` phase
    Pong,
    /// Packet to respond to `AddResourcePack` or `RemoveResourcePack` to tell the server how the
    /// requested Action was processed and if it was successful
    ResoucePackResponse(ResoucePackResponse),
}
impl ServerboundPackets {
    /// A read function to read a Serverbound packet from and `BufReader<ReadHalf>`
    ///
    /// # Returns
    ///
    /// Returns a result containing the read packet. If an error occurs it gets returned
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
