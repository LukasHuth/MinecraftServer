use crate::{datatypes::packet_implementation::packets::Packet, DatastructError};

pub struct LoginAcknowledged {
}
impl Packet for LoginAcknowledged {
    fn read(_: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<LoginAcknowledged, DatastructError> where Self: Sized {
        unreachable!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Result<LoginAcknowledged, DatastructError> where Self: Sized {
        unreachable!()
    }
}
