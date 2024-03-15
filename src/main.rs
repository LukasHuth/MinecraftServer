extern crate uuid;
use std::{net::TcpListener, sync::{Arc, Mutex}};

use server_structure::{server::Server, client_handler::ClientHandler};

fn main() {
    packet_api::test();
    let listener = TcpListener::bind("82.165.0.111:25565").unwrap();
    // let player_count = Arc::new(Mutex::new(0));
    // let players = Arc::new(Mutex::new(Vec::<Player>::new()));
    let server = Arc::new(Mutex::new(Server::new()));
    let mut threads = vec![];
    for stream in listener.incoming() {
        // let player_count_clone = Arc::clone(&player_count);
        let server_clone = Arc::clone(&server);
        let thread = std::thread::spawn(move || {
            ClientHandler::run(stream, server_clone)
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
