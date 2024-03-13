use std::{net::TcpStream, sync::{Mutex, Arc}, io::{Error, Write as _}};

use packet_api::datatypes::packet_implementation::{PacketAnalyzer, Packet, packets::{LegacyPong, StatusResponse, PongResponse, Packet as _, LoginEncriptionRequest, LoginSuccess}};

use crate::Server;

pub fn handle_client(stream: Result<TcpStream, Error>, server: Arc<Mutex<Server>>) {
    let mut client = stream.unwrap();
    let mut client_clone = client.try_clone().expect("Couldnt create Connection clone");
    let mut client_connection = PacketAnalyzer::new(&mut client_clone);
    loop {
        let packet = client_connection.next_packet();
        match packet {
            Packet::LegacyPing(_) => {
                let server = server.lock().unwrap();
                let player_count = server.players.len();
                let settings = &server.server_settings;
                let pong = LegacyPong::new(settings.version, settings.motd, player_count as u16, settings.max_players);
                let data = pong.to_bytes();
                client.write_all(&data).expect("Could not send Pong Packet");
                client.shutdown(std::net::Shutdown::Both).expect("Could not close the connection");
                break;
            },
            Packet::StatusRequest => {
                let server = server.lock().expect("Could not reserve server info");
                let settings = &server.server_settings;
                let response = StatusResponse::new(
                    settings.version.to_string(),
                    settings.protocol_version,
                    settings.max_players,
                    server.players.clone(),
                    settings.motd.to_string(),
                );
                let data = response.to_bytes();
                client.write_all(&data).expect("Could not send Status Response Packet");
            },
            Packet::LoginStart(login_start) => {
                let server = server.lock().expect("Could not reserver server info");
                let settings = server.server_settings.clone();
                if settings.offline_mode {
                    client_connection.skip_authentication();
                    let login_success = LoginSuccess::new(login_start.uuid, login_start.name);
                    let data = login_success.to_bytes();
                    println!("data: {:?}", data);
                    client.write_all(&data).expect("Could not sent login success packet");
                    continue;
                }
            },
            Packet::Ping(ping) => {
                let pong = PongResponse::new(ping);
                let data = pong.to_bytes();
                client.write_all(&data).expect("Could not send the Pong Response Packet");
            },
            Packet::None => (),
        }
    }
}
