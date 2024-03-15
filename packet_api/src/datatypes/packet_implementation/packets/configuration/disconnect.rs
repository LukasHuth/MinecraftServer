use crate::datatypes::{datastructs::JSONTextComponent, packet_implementation::packets::Packet};

pub struct ConfigurationDisconnect {
    pub reason: JSONTextComponent,
}
impl Packet for ConfigurationDisconnect {
    fn read(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        unreachable!()
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Option<Self> where Self: Sized {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.push(0x01);
        self.reason.write(&mut res);
        res
    }
}
