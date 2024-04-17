//! This create implements the important functions, needed to run a minecraft server.
//! This includes:
//! - `Server`: object able to handle the world and enable communication between players
//! - `Server::start()`: static function that makes it easy to listen to a port and send player
//! connections to their seperate Thread using `ConnectionHandler`
//! - `ConnectionHandler`: used to handle the player socket
#[deny(missing_docs)]

/// Module that specifies server specific data and functions
pub mod server;
/// Module that specifies player specific structs
pub mod player;
/// Module that specifies structs and functions used to handle player sockets
pub mod connection_handler;
/// Module that specifies structs and functions to easily load and store server configurations
pub mod config;
