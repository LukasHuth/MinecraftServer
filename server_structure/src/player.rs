use std::{collections::VecDeque, sync::{Mutex, Arc}};

use packet_api::datatypes::{datastructs::UUID, packet_implementation::packets::Packet};

#[derive(Clone)]
pub struct Player {
    pub username: std::string::String,
    pub uuid: UUID,
    pub queue: VecDeque<Arc<Mutex<dyn Packet>>>,
}
impl Player {
    pub fn new(username: String, uuid: UUID) -> Self {
        Self { username, uuid, queue: VecDeque::new() }
    }
    pub fn send_packet(&mut self, packet: Arc<Mutex<dyn Packet>>) {
        self.queue.push_back(packet);
    }
    pub fn send_message(&mut self, _message: &String) {
        // TODO: create Message Packet
    }
    pub fn into_packet_player(&self) -> packet_api::datatypes::datastructs::player::Player {
        packet_api::datatypes::datastructs::player::Player { username: self.username.clone(), uuid: self.uuid.clone(), queue: self.queue.clone() }
    }
}
impl From<packet_api::datatypes::datastructs::player::Player> for Player {
    fn from(value: packet_api::datatypes::datastructs::player::Player) -> Self {
        Self {username: value.username.clone(), uuid: value.uuid.clone(), queue: value.queue.clone()}
    }
}
