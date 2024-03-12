use std::{net::{TcpStream, TcpListener}, io::{BufReader, Read, Write}};

use super::{UncompressedPacket, necesary::Necesary, VarInt, ByteArray, Byte, String, UnsignedShort, Int};

const SERVER_VERSION: &str = "1.20.1";
const MOTD: &str = "A Rust Minecraft Server";
const MAX_PLAYERS: u16 = 20;

impl ByteArray {
    fn read<'a, I>(arr: &mut I, length: u64) -> Self where I: Iterator<Item = &'a u8> {
        let mut data = vec![];
        for _ in 0..length {
            data.push(Byte::read(arr));
        }
        Self(data)
    }
    fn to_vector(&self) -> Vec<u8> {
        self.0.iter().map(|b|b.0 as u8).collect()
    }
}
impl String {
    fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let mut str = Self::new(std::string::String::new());
        str.read(arr, None);
        str
    }
}
impl UnsignedShort {
    fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let mut data: u16 = 0;
        for _ in 0..2 {
            data <<= 8;
            data |= *arr.next().expect("Expected bytes to read") as u16;
        }
        UnsignedShort(data)
    }
}
pub enum HandshakeState {
    Status, Login
}
impl HandshakeState {
    fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let data = VarInt::read(arr);
        match data.get_value() {
            0 => Self::Status,
            1 => Self::Login,
            _ => panic!("Unknown handshake state enum"),
        }
    }
}
impl Int {
    fn read<'a, I>(arr: &mut I) -> Self where I: Iterator<Item = &'a u8> {
        let mut d = Int::new(0);
        d.read(arr, None);
        d
    }
}
pub enum Answer {
    None,
    Some(Vec<u8>)
}
pub struct Server {
    client: TcpStream,
    port: usize,
    address: &'static str,
    run: bool,
    players: u16,
}
trait TcpRead {
    fn read_stream(stream: &mut TcpStream) -> Self;
}
const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BITS: u8 = 0x80;
impl TcpRead for VarInt {
    fn read_stream(stream: &mut TcpStream) -> Self {
        let mut position = 0;
        let mut value: i32 = 0;
        loop {
            let mut bytes = [0u8; 1];
            stream.read_exact(&mut bytes);
            let current_byte = bytes[0];
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;
            if (current_byte&CONTINUE_BITS) == 0 { break; }
            position += 7;
            assert!(position < 32, "VarInt is too big");
        }
        let bytes = position / 7 + 1;
        Self(bytes, value)
    }
}
fn writeString(arr: &mut Vec<u8>, str: &str) {
    for char in str.chars() {
        arr.push(0u8);
        arr.push(char as u8);
    }
}
impl Server {
    pub fn new(port: usize, address: &'static str, client: TcpStream) -> Self {
        Self { client, port, address, run: true, players: 0, }
    }
    fn handlePing(&mut self) {
    }
    pub fn run(&mut self) {
        while self.run {
            let mut buffer = [0; 3];
            self.client.peek(&mut buffer);
            if buffer[0] == 0 { continue; }
            println!("{:?}|{}|{}|{}", buffer, buffer[0] == 0xFE, buffer[1] == 0x01, buffer[2] == 0xFA);
            if buffer[0] == 0xFE && buffer[1] == 0x01 && buffer[2] == 0xFA {
                let mut bytes = [0; 1024];
                self.client.read(&mut bytes);
                let mut response: Vec<u8> = Vec::new();
                response.push(0xFF);
                let players_str = format!("{}", self.players);
                let max_players_str = format!("{}", MAX_PLAYERS);
                let protocol_str = format!("{}", 127);
                let length = 2 * ( 2 + 1 + protocol_str.len() + 1 + SERVER_VERSION.len() + 1 + MOTD.len() + 1 + players_str.len() + 1 + max_players_str.len());
                response.push((length >> 8) as u8);
                response.push((length & 0xFF) as u8);
                writeString(&mut response, "§1");
                response.push(0);
                writeString(&mut response, &protocol_str);
                response.push(0);
                writeString(&mut response, SERVER_VERSION);
                response.push(0);
                writeString(&mut response, MOTD);
                response.push(0);
                writeString(&mut response, &players_str);
                response.push(0);
                writeString(&mut response, &max_players_str);
                self.client.write_all(&response);
                self.client.shutdown(std::net::Shutdown::Both);
                println!("Got Ping");
                self.handlePing();
                continue;
            }
            let length = VarInt::read_stream(&mut self.client);
            let packet_id = VarInt::read_stream(&mut self.client);
            let mut data: Vec<u8> = Vec::new();
            data.resize((length.get_value() - packet_id.0 as i32) as usize, 0);
            let data: &mut [u8] = &mut data;
            self.client.read_exact(data);
            let up = UncompressedPacket::new(length, packet_id, data);
            up.handle(self);
        }
    }
}
impl UncompressedPacket {
    pub fn new(length: VarInt, packet_id: VarInt, data: &[u8]) -> Self {
        let data: Vec<Byte> = data.iter().map(|b|Byte(*b as i8)).collect();
        let data = ByteArray(data);
        Self { length, packet_id, data }
    }

    fn write(&self, arr: &mut Vec<u8>) {
        todo!()
    }
    fn handle_handshake(&self, server: &mut Server) -> Answer {
        let arr = self.data.to_vector();
        println!("{:?}", arr);
        let mut arr = arr.iter();
        let a = Byte::read(&mut arr);
        println!("a: {}", a.0);
        let pi = VarInt::read(&mut arr);
        println!("pi: {}", pi.0);
        let length = UnsignedShort::read(&mut arr);
        println!("length: {}", length.0);
        let protocol_version = VarInt::read(&mut arr);
        let server_address = String::read(&mut arr);
        // let port = UnsignedShort::read(&mut arr);
        let port = Int::read(&mut arr);
        // let state = HandshakeState::read(&mut arr);
        println!("hello");
        // TODO: Set State
        Answer::None
    }

    fn handle(&self, server: &mut Server) -> Answer {
        match self.packet_id.get_value() {
            0xFE => self.handle_handshake(server),
            _ => unimplemented!("This packet is currently unimplemented"),
        }
    }
}
