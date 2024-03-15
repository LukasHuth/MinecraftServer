use std::{io::{BufReader, Read}, net::TcpStream};

use crate::datatypes::packet_implementation::{bytes_to_short, packets::Packet};

pub struct LegacyPing {}
impl LegacyPing {
    pub fn new() -> Self {
        Self { }
    }
}
impl Packet for LegacyPing {
    fn read(stream: &mut BufReader<&mut TcpStream>) -> Option<Self> where Self: Sized {
        let mut info_bytes = [0;3];
        stream.read_exact(&mut info_bytes).expect("Could not read the info bytes");
        let mut string_length = [0;2];
        stream.read_exact(&mut string_length).expect("Failed to load string length");
        let length = bytes_to_short(string_length) * 2;
        let mut string = vec![0;length as usize];
        stream.read_exact(&mut string).expect("Failed to load String");
        let mut following_size = [0;2];
        stream.read_exact(&mut following_size).expect("Failed to load followup size");
        let mut data = vec![0; bytes_to_short(following_size) as usize];
        stream.read_exact(&mut data).expect("Failed to load to remaining legacy ping data");
        Some(Self{})
    }
    fn to_bytes(&self) -> Vec<u8> {
        unreachable!("Legacy Ping should never be decoded to bytes")
    }

    fn read_length(_stream: &mut BufReader<&mut TcpStream>, _length: crate::datatypes::datastructs::VarInt) -> Option<Self> where Self: Sized {
        unreachable!()
    }
}
