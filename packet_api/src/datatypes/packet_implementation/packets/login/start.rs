use crate::datatypes::datastructs::{String, UUID, necesary::Necesary as _};

use crate::datatypes::packet_implementation::packets::Packet;

pub struct LoginStart {
    pub name: String,
    pub uuid: UUID,
}
impl Packet for LoginStart {
    fn read(stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        let name = String::read(stream, None);
        let uuid = UUID::read(stream, None);
        Some(Self {
            name,
            uuid
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("Not needed")
    }
}
