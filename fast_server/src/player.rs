use std::{collections::VecDeque, sync::{Arc, Mutex}};

use fast_protocol::datatypes::packets::ClientboundPackets;
use tokio::sync::mpsc;

pub enum PlayerMessages {
}

#[derive(Clone)]
pub struct Player {
    pub uuid: u128,
    pub username: String,
    pub sender: mpsc::Sender<PlayerMessages>,
}
