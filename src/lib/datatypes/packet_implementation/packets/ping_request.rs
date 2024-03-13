use crate::datatypes::datastructs::{Long, necesary::Necesary as _};

use super::Packet;

pub struct PingRequest {
    pub data: Long,
}
impl Packet for PingRequest {
    fn read(stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        let data = Long::read(stream, None);
        Some(Self{ data })
    }

    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("This packet should never be sent")
    }
}
