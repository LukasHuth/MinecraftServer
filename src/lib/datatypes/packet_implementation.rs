use std::{net::TcpStream, io::{BufReader, Read, BufRead}};
use self::packets::LegacyPing;
use super::datastructs::{VarInt, String, UnsignedShort};
use crate::datatypes::{datastructs::necesary::Necesary as _, packet_implementation::packets::{Handshake, Packet as _}};

pub mod packets;

const CURRENT_PROTOCOL_VERSION: i32 = 765;

#[derive(Debug)]
pub enum ClientHandlerState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}
pub enum Answer {
    None,
    Some(Vec<u8>)
}
pub struct PacketAnalyzer<'a> {
    reader: BufReader<&'a mut TcpStream>,
    status: ClientHandlerState,
}
const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BITS: u8 = 0x80;
impl VarInt {
    fn read_reader(stream: &mut BufReader<&mut TcpStream>) -> Self {
        let mut position = 0;
        let mut value: i32 = 0;
        loop {
            let mut bytes = [0u8; 1];
            stream.read_exact(&mut bytes).expect("Failed to read bytes for the varint");
            let current_byte = bytes[0];
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 32, "VarInt is too big");
        }
        let bytes = position / 7 + 1;
        Self::new(bytes, value)
    }
}
pub enum Packet {
    LegacyPing(LegacyPing),
    StatusRequest,
    None,
}
fn bytes_to_short(arr: [u8;2]) -> u16 {
    ((arr[0] as u16) << 8) + arr[1] as u16
}
impl<'a> PacketAnalyzer<'a> {
    pub fn new(client: &'a mut TcpStream) -> Self {
        let reader = BufReader::new(client);
        Self { reader, status: ClientHandlerState::Handshaking }
    }
    fn next_handshake_packet(&mut self) -> Packet {
        let mut peek_buffer = [0; 3];
        match self.reader.fill_buf() {
            Ok(buf) => {
                let bytes_to_peek = std::cmp::min(buf.len(), 3);
                peek_buffer[..bytes_to_peek].copy_from_slice(&buf[..bytes_to_peek]);
                if bytes_to_peek < 3 { panic!("Too few bytes to peek"); }
            },
            Err(_) => panic!("Error occured on peek"),
        }
        if peek_buffer[0] == 0xFE && peek_buffer[1] == 0x01 && peek_buffer[2] == 0xFA {
            println!("Legacy Ping");
            return Packet::LegacyPing(LegacyPing::read(&mut self.reader).expect("Could not read the Legacy Ping request"));
        }
        println!("Normal Ping");
        let handshake_packet = Handshake::read(&mut self.reader).expect("Could not read the handshake packet");
        if handshake_packet.protocol_version.get_value() < CURRENT_PROTOCOL_VERSION { panic!("Client is using outdated packet protocol "); }
        match handshake_packet.next_state.get_value() {
            1 => self.status = ClientHandlerState::Status,
            2 => self.status = ClientHandlerState::Login,
            _ => panic!("Unknown enum id")
        }
        println!("{:?}", self.status);
        return Packet::None;
    }
    fn next_login_packet(&mut self) -> Packet {
        Packet::None
    }
    fn next_status_packet(&mut self) -> Packet {
        let length = VarInt::read(&mut self.reader, None);
        let packet_id = VarInt::read(&mut self.reader, None);
        match packet_id.get_value() {
            0 => Packet::StatusRequest,
            1 => unimplemented!(),
            _ => Packet::None,
        }
    }
    fn next_configuration_packet(&mut self) -> Packet {
        Packet::None
    }
    fn next_play_packet(&mut self) -> Packet {
        Packet::None
    }
    pub fn next_packet(&mut self) -> Packet {
        match self.status {
            ClientHandlerState::Handshaking => self.next_handshake_packet(),
            ClientHandlerState::Status => self.next_status_packet(),
            ClientHandlerState::Login => self.next_login_packet(),
            ClientHandlerState::Configuration => self.next_configuration_packet(),
            ClientHandlerState::Play => self.next_play_packet(),
        }
    }
}
