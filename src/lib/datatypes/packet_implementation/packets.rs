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
mod ping_request;
mod pong_response;
mod login_disconnect;
mod login_encription_request;
mod login_success;
mod login_start;
mod login_encription_response;
// plugin messages on login ignored
mod login_acknowledged;

// exporting

pub use self::legacy_ping::LegacyPing;
pub use self::legacy_pong::LegacyPong;
pub use self::handshake::Handshake;
pub use self::status_response::StatusResponse;
pub use self::ping_request::PingRequest;
pub use self::pong_response:: PongResponse;
pub use self::login_disconnect::LoginDisconnect;
pub use self::login_encription_request::LoginEncriptionRequest;
pub use self::login_success::LoginSuccess;
pub use self::login_start::LoginStart;
pub use self::login_encription_response::LoginEncriptionResponse;
pub use self::login_acknowledged::LoginAcknowledged;
