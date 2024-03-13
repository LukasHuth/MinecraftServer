use std::{net::TcpListener, sync::{Arc, Mutex}};

use client_handling::handle_client;

use server::Server;

mod client_handling;
mod server;


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
            handle_client(stream, server_clone)
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
