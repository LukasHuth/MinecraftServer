#![feature(str_from_utf16_endian)]
use std::net::TcpListener;

use datatypes::necesary::Necesary;
use crate::datatypes::{Identifier, VarInt, UncompressedPacket, Packet, CompressedPacket, packet_implementation::Server};

mod datatypes;
#[cfg(test)]
mod test;

enum PacketState {
    Compressed, Uncompressed
}
pub fn test() {
    let test: [u8;5] = [0x80, 0x80, 0x80, 0x80, 0x08];
    let mut vl = VarInt::new(0);
    let mut ti = test.iter();
    vl.read(&mut ti, None);
    println!("{}|2147483647", vl.get_value());
    let mut arr = vec![];
    vl.write(&mut arr);
    println!("{:?}", arr);
    let ident = Identifier::new(String::from("hallo"));
    println!("{}", ident.get_value());
    let packet_type = PacketState::Uncompressed;
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        let client = stream.unwrap();
        let mut server = Server::new(25565, "127.0.0.1", client);
        server.run();
    }
}
