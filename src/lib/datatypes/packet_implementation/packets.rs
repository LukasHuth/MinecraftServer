use std::{net::TcpStream, io::BufReader};


pub trait Packet {
    fn read(stream: &mut BufReader<&mut TcpStream>) -> Option<Self> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
pub struct LegacyPing {}
impl LegacyPing {
    pub fn new() -> Self {
        Self { }
    }
}
pub struct Pong<'a> {
    protocol_version: u8,
    server_version: &'a str,
    motd: &'a str,
    player_count: u16,
    max_players: u16,
}
impl<'a> Pong<'a> {
    pub fn new(server_version: &'a str, motd: &'a str,player_count: u16, max_players: u16) -> Self {
        let protocol_version = 127;
        Self {protocol_version, server_version, motd, player_count, max_players}
    }
}
fn write_string(arr: &mut Vec<u8>, str: &str) {
    for char in str.chars() {
        arr.push(0);
        arr.push(char as u8);
    }
}
impl Packet for LegacyPing {
    fn read(_: &mut BufReader<&mut TcpStream>) -> Option<Self> where Self: Sized {
        todo!()
    }
    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("Legacy Ping should never be decoded to bytes")
    }
}
impl<'a> Packet for Pong<'a> {
    fn read(_: &mut BufReader<&mut TcpStream>) -> Option<Self> where Self: Sized {
        unreachable!("Legacy Pong should never be read")
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        let info_str = "§1";
        let protocol_str = format!("{}", self.protocol_version);
        let player_count_str = format!("{}", self.player_count);
        let max_players_str = format!("{}", self.max_players);
        let length = info_str.len() + 1 + protocol_str.len() + 1 + self.server_version.len() + 1 + self.motd.len() + 1 + player_count_str.len() + 1 + max_players_str.len() - 1;
        result.push(0xFF);
        result.push((length >> 8) as u8);
        result.push(length as u8);
        write_string(&mut result, info_str);
        result.push(0);
        result.push(0);
        write_string(&mut result, &protocol_str);
        result.push(0);
        result.push(0);
        write_string(&mut result, self.server_version);
        result.push(0);
        result.push(0);
        write_string(&mut result, self.motd);
        result.push(0);
        result.push(0);
        write_string(&mut result, &player_count_str);
        result.push(0);
        result.push(0);
        write_string(&mut result, &max_players_str);
        result
    }
}
