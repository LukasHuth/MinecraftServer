use std::{net::TcpStream, io::{BufReader, Read, BufRead}};

use self::packets::LegacyPing;

use super::{necesary::Necesary, VarInt};

pub enum HandshakeState {
    Status, Login
}
pub enum Answer {
    None,
    Some(Vec<u8>)
}
pub struct PacketAnalyzer<'a> {
    reader: BufReader<&'a mut TcpStream>,
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
        Self(bytes, value)
    }
}
pub enum Packet {
    LegacyPing(LegacyPing),
    None,
}
fn bytes_to_short(arr: [u8;2]) -> u16 {
    ((arr[0] as u16) << 8) + arr[1] as u16
}
pub mod packets;
impl<'a> PacketAnalyzer<'a> {
    pub fn new(client: &'a mut TcpStream) -> Self {
        let reader = BufReader::new(client);
        Self { reader }
    }
    fn handle_legacy_ping(&mut self) -> Packet {
        let mut info_bytes = [0;3];
        self.reader.read_exact(&mut info_bytes).expect("Could not read the info bytes");
        let mut string_length = [0;2];
        self.reader.read_exact(&mut string_length).expect("Failed to load string length");
        let length = bytes_to_short(string_length) * 2;
        let mut string = vec![0;length as usize];
        self.reader.read_exact(&mut string).expect("Failed to load String");
        let mut following_size = [0;2];
        self.reader.read_exact(&mut following_size).expect("Failed to load followup size");
        let mut data = vec![0; bytes_to_short(following_size) as usize];
        self.reader.read_exact(&mut data).expect("Failed to load to remaining legacy ping data");
        let ping = LegacyPing::new();
        println!("Got Ping");
        Packet::LegacyPing(ping)
    }
    pub fn next_packet(&mut self) -> Packet {
        loop {
            let mut peek_buffer = [0; 3];
            match self.reader.fill_buf() {
                Ok(buf) => {
                    let bytes_to_peek = std::cmp::min(buf.len(), 3);
                    peek_buffer[..bytes_to_peek].copy_from_slice(&buf[..bytes_to_peek]);
                    if bytes_to_peek < 3 { continue; }
                },
                Err(_) => continue,
            }
            if peek_buffer[0] == 0xFE && peek_buffer[1] == 0x01 && peek_buffer[2] == 0xFA {
                return self.handle_legacy_ping();
            }
            let length = VarInt::read_reader(&mut self.reader);
            let packet_id = VarInt::read_reader(&mut self.reader);
            let mut data = vec![0;(length.get_value() - packet_id.0 as i32) as usize];
            self.reader.read_exact(&mut data).expect("Failed to load packet Data");
            return Packet::None;
        }
    }
}
