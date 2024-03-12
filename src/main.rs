use std::{net::TcpListener, sync::{Arc, Mutex}, io::Write};

use packet_api::datatypes::packet_implementation::{PacketAnalyzer, Packet, packets::LegacyPong};

use packet_api::datatypes::packet_implementation::packets::Packet as _;

const SERVER_VERSION: &str = "1.20.4";
const MOTD: &str = "A Cool Rust Server";
const MAX_PLAYERS: u16 = 100;

fn main() {
    let listener = TcpListener::bind("82.165.0.111:25565").unwrap();
    let player_count = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for stream in listener.incoming() {
        let player_count_clone = Arc::clone(&player_count);
        let thread = std::thread::spawn(move || {
            let mut client = stream.unwrap();
            let mut client_connection = PacketAnalyzer::new(&mut client);
            loop {
                let packet = client_connection.next_packet();
                match packet {
                    Packet::LegacyPing(_) => {
                        let pong = LegacyPong::new(SERVER_VERSION, MOTD, *player_count_clone.lock().unwrap(), MAX_PLAYERS);
                        let data = pong.to_bytes();
                        client.write_all(&data).expect("Could not send Pong Packet");
                        client.shutdown(std::net::Shutdown::Both).expect("Could not close the connection");
                        break;
                    },
                    Packet::StatusRequest => {
                    },
                    Packet::None => (),
                }
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
