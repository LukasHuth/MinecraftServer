use crate::DatastructError;
use crate::datatypes::datastructs::{String, UUID, necesary::Necesary as _};

use crate::datatypes::packet_implementation::packets::Packet;

pub struct LoginStart {
    pub name: String,
    pub uuid: UUID,
}
impl Packet for LoginStart {
    fn read(stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<Self, DatastructError> where Self: Sized {
        let name = *String::read(stream, None)?;
        let uuid = *UUID::read(stream, None)?;
        Ok(Self {
            name,
            uuid
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("Not needed")
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Result<Self, DatastructError> where Self: Sized {
        unreachable!()
    }
}
