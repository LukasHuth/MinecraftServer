use crate::{datatypes::{datastructs::{Identifier, ByteArray, necesary::Necesary}, packet_implementation::packets::Packet}, DatastructError};

pub struct ConfigurationClientboundPluginMessage {
    pub channel: Identifier,
    pub data: ByteArray,
}
impl Packet for ConfigurationClientboundPluginMessage {
    fn read(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<ConfigurationClientboundPluginMessage, DatastructError> where Self: Sized {
        unreachable!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        unreachable!()
    }

    fn read_length(stream: &mut std::io::BufReader<&mut std::net::TcpStream>, length: crate::datatypes::datastructs::VarInt)
        -> Result<ConfigurationClientboundPluginMessage, DatastructError>
            where Self: Sized {
        let identifier = *Identifier::read(stream, None)?;
        let length = length.get_value() - 1 - identifier.get_value().as_bytes().len() as i32;
        let data = *ByteArray::read(stream, Some(length as u64))?;
        Ok(Self { channel: identifier, data })
    }
}
