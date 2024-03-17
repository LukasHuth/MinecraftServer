use crate::datatypes::datastructs::{Long, necesary::Necesary, VarInt};

use super::ping_request::PingRequest;

use crate::datatypes::packet_implementation::packets::Packet;
use crate::{TestNeccessaryTrait, DatastructError};

pub struct PongResponse {
    data: Long,
}
impl Packet for PongResponse {
    fn read(_: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<PongResponse, DatastructError> where Self: Sized {
        unreachable!("This packet should never be read")
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut response = vec![];
        let packet_id = VarInt::new(0x01);
        let packet_size = VarInt::new(packet_id.get_bytes() as i32 + 8);
        packet_size.write(&mut response);
        packet_id.write(&mut response);
        println!("data: {}", self.data.get_value());
        self.data.write(&mut response);
        response
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: VarInt) -> Result<PongResponse, DatastructError> where Self: Sized {
        unreachable!()
    }
}
impl PongResponse {
    pub fn new(pr: PingRequest) -> Self {
        Self { data: pr.data }
    }
}