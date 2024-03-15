use std::{io::{BufReader, BufRead}, net::TcpStream};
extern crate uuid;

use self::uuid::Uuid;

use self::packets::{LegacyPing, PingRequest, LoginStart};
use super::datastructs::{VarInt, player::Player, UUID, String};
use crate::datatypes::{datastructs::necesary::Necesary, packet_implementation::packets::{Handshake, Packet as _}};
use crate::TestNeccessaryTrait;

pub mod packets;

const CURRENT_PROTOCOL_VERSION: i32 = 765;

#[derive(Debug, Clone)]
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
    login_state: LoginState,
    pub player: Option<Player>,
    pub id: Uuid,
}
#[derive(Clone)]
enum LoginState {
    Start,
    EncRes,
    LogAck,
}
pub enum Packet {
    LegacyPing(LegacyPing),
    StatusRequest,
    Ping(PingRequest),
    None,
    LoginStart(LoginStart),
    LoginAck,
}
fn bytes_to_short(arr: [u8;2]) -> u16 {
    ((arr[0] as u16) << 8) + arr[1] as u16
}
impl<'a> PacketAnalyzer<'a> {
    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }
    pub fn send_message(&mut self, _msg: &str) {
        // TODO:
    }
    pub fn new(client: &'a mut TcpStream) -> Self {
        let reader = BufReader::new(client);
        Self {
            reader,
            status: ClientHandlerState::Handshaking,
            handled_status: false,
            login_state: LoginState::Start,
            player: None,
            id: Uuid::new_v4(),
        }
    }
    pub fn skip_authentication(&mut self) {
        self.login_state = LoginState::LogAck;
    }
    fn switch_to_login(&mut self) {
        self.login_state =LoginState::Start;
        self.status = ClientHandlerState::Login;
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
            2 => self.switch_to_login(),
            _ => panic!("Unknown enum id")
        }
        println!("{:?}", self.status);
        return Packet::None;
    }
    fn next_login_packet(&mut self) -> Packet {
        let _length = VarInt::read(&mut self.reader, None);
        let packet_id = VarInt::read(&mut self.reader, None);
        match self.login_state {
            LoginState::Start => {
                println!("Login start revieved");
                assert_eq!(packet_id.get_value(), 0x00);
                self.login_state = LoginState::EncRes;
                Packet::LoginStart(LoginStart::read(&mut self.reader).expect("Cound not load the login start packet"))
            },
            LoginState::EncRes => {
                assert_eq!(packet_id.get_value(), 0x01);
                println!("EncRes");
                self.login_state = LoginState::LogAck;
                Packet::None
            },
            LoginState::LogAck => {
                println!("Login Ack Recieved");
                assert_eq!(packet_id.get_value(), 0x03);
                self.status = ClientHandlerState::Configuration;
                Packet::LoginAck
            }
        }
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
        let _length = VarInt::read(&mut self.reader, None);
        let packet_id = VarInt::read(&mut self.reader, None);
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
    pub fn create_player(&mut self, username: String, uuid: UUID) {
        let player = Player::new(username.get_value().clone(), uuid);
        self.player = Some(player);
    }
}
