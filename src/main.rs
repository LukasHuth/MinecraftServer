use std::{net::TcpListener, sync::{Arc, Mutex}, io::Write};

use packet_api::datatypes::{packet_implementation::{PacketAnalyzer, Packet, packets::{LegacyPong, StatusResponse, PongResponse}}, datastructs::Player};

use packet_api::datatypes::packet_implementation::packets::Packet as _;

const SERVER_VERSION: &str = "1.20.4";
const MOTD: &str = "A Cool Rust Server";
const MAX_PLAYERS: u16 = 100;
const PROTOCOL_VERSION: u16 = 765;

fn main() {
    let listener = TcpListener::bind("82.165.0.111:25565").unwrap();
    // let player_count = Arc::new(Mutex::new(0));
    let players = Arc::new(Mutex::new(Vec::<Player>::new()));
    let mut threads = vec![];
    for stream in listener.incoming() {
        // let player_count_clone = Arc::clone(&player_count);
        let players_clone = Arc::clone(&players);
        let thread = std::thread::spawn(move || {
            let mut client = stream.unwrap();
            let mut client_clone = client.try_clone().expect("Couldnt create Connection clone");
            let mut client_connection = PacketAnalyzer::new(&mut client_clone);
            loop {
                let packet = client_connection.next_packet();
                match packet {
                    Packet::LegacyPing(_) => {
                        let player_count = players_clone.lock().unwrap().len();
                        let pong = LegacyPong::new(SERVER_VERSION, MOTD, player_count as u16, MAX_PLAYERS);
                        let data = pong.to_bytes();
                        client.write_all(&data).expect("Could not send Pong Packet");
                        client.shutdown(std::net::Shutdown::Both).expect("Could not close the connection");
                        break;
                    },
                    Packet::StatusRequest => {
                        let response = StatusResponse::new(
                            SERVER_VERSION.to_string(),
                            PROTOCOL_VERSION,
                            MAX_PLAYERS,
                            players_clone.lock().unwrap().clone(),
                            MOTD.to_string(),
                        );
                        let data = response.to_bytes();
                        client.write_all(&data).expect("Could not send Status Response Packet");
                    },
                    Packet::Ping(ping) => {
                        let pong = PongResponse::new(ping);
                        let data = pong.to_bytes();
                        client.write_all(&data).expect("Could not send the Pong Response Packet");
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
