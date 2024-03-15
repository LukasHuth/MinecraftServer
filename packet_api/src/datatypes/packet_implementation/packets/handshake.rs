use crate::datatypes::datastructs::{VarInt, String, UnsignedShort, necesary::Necesary as _};

use super::Packet;


pub struct Handshake {
    pub length: VarInt,
    pub packet_id: VarInt,
    pub protocol_version: VarInt,
    pub next_state: VarInt,
}
impl Packet for Handshake {
    fn read(stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        let length = VarInt::read(stream, None);
        // println!("length: {}", length.get_value());
        let packet_id = VarInt::read(stream, None);
        // println!("packet id: {}", packet_id.get_value());
        if packet_id.get_value() != 0x00 { return None; }
        let protocol_version = VarInt::read(stream, None);
        // println!("protocol version: {}", protocol_version.get_value());
        let _server_address = String::read(stream, None);
        // println!("server address: {}", _server_address.get_value());
        let _server_port = UnsignedShort::read(stream, None);
        let next_state = VarInt::read(stream, None);
        Some(Self {length, packet_id, next_state, protocol_version })
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: VarInt) -> Option<Self> where Self: Sized {
        unimplemented!()
    }
}
