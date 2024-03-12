use std::{net::TcpStream, io::BufReader};


pub trait Packet {
    fn read(stream: &mut BufReader<&mut TcpStream>) -> Option<Self> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
fn write_string(arr: &mut Vec<u8>, str: &str) {
    for char in str.chars() {
        arr.push(0);
        arr.push(char as u8);
    }
}
mod legacy_ping;
mod legacy_pong;
mod handshake;
mod status_response;

// exporting

pub use self::legacy_ping::LegacyPing;
pub use self::legacy_pong::LegacyPong;
pub use self::handshake::Handshake;
