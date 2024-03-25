use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

use crate::utils::{DataWriter, write_bytes, consume_utf16be_char, PacketReader, DataReader};
use crate::errors::Error;
use crate::datatypes::datatype_definition::{VarInt, UnsignedShort, Enum, important_enums::HandshakeNextState, String};

pub struct LegacyPongPacket {
    server_version: std::string::String,
    motd: std::string::String,
    current_players: u16,
    max_players: u16
}
pub struct HandshakePacket {
    pub protocol: VarInt,
    pub address: String,
    pub port: UnsignedShort,
    pub next_state: Enum<HandshakeNextState, VarInt>
}
pub struct LegacyPingPacket;
impl LegacyPongPacket {
    pub fn new(server_version: std::string::String, motd: std::string::String, current_players: u16, max_players: u16) -> Self {
        Self{ server_version, motd, current_players, max_players }
    }
}
impl DataWriter for LegacyPongPacket {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> crate::errors::Result<()> {
        let mut d = Vec::new();
        let data = format!("{}\0{}\0{}\0{}\0{}", 127, self.server_version, self.motd, self.current_players, self.max_players);
        let length = data.len() + 3; // +3 because the two beginning chars plus zero
        let data = data.encode_utf16().collect::<Vec<_>>();
        let data: Vec<u8> = data.iter().map(|i| [((i>>8)&0xFF) as u8, (i&0xFF) as u8]).flatten().collect();
        // let length = data.len();
        // let length = data.len();
        let length = [(length << 8) as u8, length as u8];
        write_bytes(&mut d, &[0xFF]).await?;
        write_bytes(&mut d, &length).await?;
        write_bytes(&mut d, &[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]).await?;
        write_bytes(&mut d, &data).await?;
        write_bytes(writer, &d).await?;
        Ok(())
    }
}
impl DataReader for LegacyPingPacket {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> crate::errors::Result<Self> {
        let mut data = [0; 3];
        match reader.read_exact(&mut data).await { Ok(_) => Ok(()), Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into()}?;
        println!("data: {:?}", data);
        let length = ((data[1] as u16) << 8) | data[2] as u16;
        let length = length;
        println!("length: {length}");
        for i in 0..length {
            println!("i: {i}");
            consume_utf16be_char(reader, line!(), file!()).await?;
        }
        println!("Chars consumed");
        let mut length = [0;2];
        match reader.read_exact(&mut length).await { Ok(_) => Ok(()), Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into()}?;
        let length = ((length[0] as u16) << 8) | length[1] as u16;
        let mut data_buf = vec![0; length as usize];
        match reader.read_exact(&mut data_buf).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into()
            }
        }
        println!("bytes to consume: {length}");
        // consume_n_bytes(reader, length as u64).await?;
        // println!("bytes consumed");
        Ok(Self)
    }
}
impl PacketReader for HandshakePacket {
    async fn read(reader: &mut (impl AsyncRead + Unpin), _length: i32, _packet_id: i32) -> crate::errors::Result<Self> {
        let protocol = VarInt::read(reader).await?;
        let address= String::read(reader).await?;
        let port = UnsignedShort::read(reader).await?;
        let next_state = Enum::read(reader).await?;
        Ok(Self{ protocol, address, port, next_state })
    }
}
