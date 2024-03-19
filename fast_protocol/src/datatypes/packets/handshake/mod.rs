use crate::{datatypes::{json_datastructures::{StatusResponseJSON, Player, Version}, datatype_definition::{VarInt, Long}}, utils::{DataWriter, write_bytes}};

pub struct LegacyPongPacket(std::string::String, std::string::String, u16, u16);
impl LegacyPongPacket {
    pub fn new(server_version: std::string::String, motd: std::string::String, current_players: u16, max_players: u16) -> Self {
        Self(server_version, motd, current_players, max_players)
    }
}
impl DataWriter for LegacyPongPacket {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let data = format!("§1\0{}\0{}\0{}\0{}\0{}", 127, self.0, self.1, self.2, self.3);
        let data = data.as_bytes().to_vec();
        let data: Vec<u8> = data.iter().map(|v|[0, *v]).flatten().collect();
        let length = data.len();
        let length = [(length << 8) as u8, length as u8];
        write_bytes(writer, &[0xFF])?;
        write_bytes(writer, &length)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
