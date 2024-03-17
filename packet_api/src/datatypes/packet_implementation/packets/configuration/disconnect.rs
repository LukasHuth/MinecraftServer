use crate::{datatypes::{datastructs::JSONTextComponent, packet_implementation::packets::Packet}, DatastructError};

pub struct ConfigurationDisconnect {
    pub reason: JSONTextComponent,
}
impl Packet for ConfigurationDisconnect {
    fn read(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<ConfigurationDisconnect, DatastructError> where Self: Sized {
        unreachable!()
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Result<ConfigurationDisconnect, DatastructError> where Self: Sized {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.push(0x01);
        // TODO:
        // self.reason.write(&mut res);
        res
    }
}
