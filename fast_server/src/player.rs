use std::{collections::VecDeque, sync::{Arc, Mutex}};

use fast_protocol::datatypes::packets::ClientboundPackets;

#[derive(Clone)]
pub struct Player {
    pub uuid: u128,
    pub username: String,
    pub queue: VecDeque<Arc<ClientboundPackets>>
}
