use std::{net::TcpStream, sync::{Mutex, Arc}, io::{Error, Write}};

use packet_api::datatypes::packet_implementation::{PacketAnalyzer, Packet, packets::{LegacyPong, StatusResponse, PongResponse, Packet as _, LoginSuccess}};

use crate::{server::{Server, ServerEvent}, player::Player};
pub struct ClientHandler;

impl ClientHandler {
    pub fn run(stream: Result<TcpStream, Error>, server: Arc<Mutex<Server>>) {
        let mut client = stream.unwrap();
        // let mut client_clone = client.try_clone().expect("Couldnt create Connection clone");
        let mut client_clone = client.try_clone().unwrap();
        let mut client_connection = PacketAnalyzer::new(&mut client_clone);
        loop {
            if let Some(mut player) = client_connection.player.clone() {
                while let Some(packet) = player.queue.pop_front() {
                    client.write_all(&packet.lock().expect("Failed to move packet throught threads").to_bytes()).expect("Failed to send packet to client");
                }
            }
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
                        server.get_players().iter().map(Player::into_packet_player).collect(),
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
                        let login_success = LoginSuccess::new(login_start.uuid.clone(), login_start.name.clone());
                        let data = login_success.to_bytes();
                        client.write_all(&data).expect("Could not sent login success packet");
                        client_connection.create_player(login_start.name, login_start.uuid);
                        continue;
                    }
                },
                Packet::LoginAck => {
                    let mut server = server.lock().expect("Could not get a mutable server instance");
                    server.trigger(ServerEvent::PlayerJoined(Player::from(client_connection.player.clone().unwrap())));
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
}
