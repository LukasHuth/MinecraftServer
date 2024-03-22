extern crate uuid;
use std::{net::TcpListener, sync::{Arc, Mutex}};

use server_structure::{server::Server, client_handler::ClientHandler};

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let _ = fast_server::server::Server::start().await;
    /*
    #[cfg(not(debug_assertions))]
    {
        packet_api::test();
        let (server, listener) = Server::start_server();
        // let listener = TcpListener::bind("82.165.0.111:25565").unwrap();
        // let player_count = Arc::new(Mutex::new(0));
        // let players = Arc::new(Mutex::new(Vec::<Player>::new()));
        // let server = Arc::new(Mutex::new(Server::new()));
        let server = Arc::new(Mutex::new(server));
        // let mut threads = vec![];
        while let Ok((stream, _)) = listener.accept() {
            let server_clone = Arc::clone(&server);
            tokio::spawn(async move {
                if let Err(err) = ClientHandler::run(stream, &server_clone).await {
                    eprintln!("Error with the client");
                    if let Some(player) = err.player {
                        server_clone.lock().unwrap().remove_player(player);
                    }
                }
            });
        }
    }
    */
    /*
    for stream in listener.incoming() {
        // let player_count_clone = Arc::clone(&player_count);
        let server_clone = Arc::clone(&server);
        let thread = std::thread::spawn(move || {
            ClientHandler::run(stream, server_clone)
        });
        threads.push(thread);
    }
    */
}
