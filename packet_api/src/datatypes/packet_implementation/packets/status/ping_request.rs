use crate::DatastructError;
use crate::datatypes::datastructs::{Long, necesary::Necesary as _};

use crate::datatypes::packet_implementation::packets::Packet;

pub struct PingRequest {
    pub data: Long,
}
impl Packet for PingRequest {
    fn read(stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<Self, DatastructError> where Self: Sized {
        let data = *Long::read(stream, None)?;
        Ok(Self{ data })
    }

    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("This packet should never be sent")
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Result<Self, DatastructError> where Self: Sized {
        unreachable!()
    }
}
