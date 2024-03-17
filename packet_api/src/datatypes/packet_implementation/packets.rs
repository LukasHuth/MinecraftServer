use std::{net::TcpStream, io::BufReader};

use crate::{datatypes::datastructs::VarInt, DatastructError};


pub trait Packet: Send {
    fn read(stream: &mut BufReader<&mut TcpStream>) -> Result<Self, DatastructError> where Self: Sized;
    fn read_length(_stream: &mut BufReader<&mut TcpStream>, _length: VarInt) -> Result<Self, DatastructError> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
fn write_string(arr: &mut Vec<u8>, str: &str) {
    for char in str.chars() {
        arr.push(0);
        arr.push(char as u8);
    }
}
macro_rules! create_packet {
    ($($packet_file_name:ident, $packet_name:tt $(,)?)+) => {
        $(
            mod $packet_file_name;
            pub use self::$packet_file_name::$packet_name;
        )*
    };
}
// plugin messages on login ignored
create_packet!(
    legacy, {LegacyPong, LegacyPing},
    handshake, Handshake,
    login, *,
    status, *,
    configuration, *,
    );