use std::{io::{BufReader, BufRead}, net::TcpStream};
use self::packets::{LegacyPing, PingRequest};
use super::datastructs::VarInt;
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
    handled_status: bool,
}
pub enum Packet {
    LegacyPing(LegacyPing),
    StatusRequest,
    Ping(PingRequest),
    None,
}
fn bytes_to_short(arr: [u8;2]) -> u16 {
    ((arr[0] as u16) << 8) + arr[1] as u16
}
impl<'a> PacketAnalyzer<'a> {
    pub fn new(client: &'a mut TcpStream) -> Self {
        let reader = BufReader::new(client);
        Self { reader, status: ClientHandlerState::Handshaking, handled_status: false }
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
        let _length = VarInt::read(&mut self.reader, None);
        let packet_id = VarInt::read(&mut self.reader, None);
        match packet_id.get_value() {
            0 => {
                if self.handled_status { panic!("Already recieved status request"); }
                self.handled_status = true;
                Packet::StatusRequest
            },
            1 => {
                Packet::Ping(PingRequest::read(&mut self.reader).expect("Could not read the ping packet"))
            },
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
        let has_data_left = self.reader.has_data_left().unwrap_or(false);
        if !has_data_left { return Packet::None; }
        match self.status {
            ClientHandlerState::Handshaking => self.next_handshake_packet(),
            ClientHandlerState::Status => self.next_status_packet(),
            ClientHandlerState::Login => self.next_login_packet(),
            ClientHandlerState::Configuration => self.next_configuration_packet(),
            ClientHandlerState::Play => self.next_play_packet(),
        }
    }
}
