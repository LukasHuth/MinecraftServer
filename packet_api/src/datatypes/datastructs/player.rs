use crate::datatypes::packet_implementation::packets::Packet;
use std::{collections::VecDeque, cell::RefCell, rc::Rc, sync::{Mutex, Arc}};

use super::UUID;

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
    pub fn send_message(&mut self, message: &String) {
        // TODO: create Message Packet
    }
}
