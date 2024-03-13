use crate::datatypes::datastructs::{String, VarInt, UUID, Boolean, necesary::Necesary};

use super::Packet;

struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: Boolean,
    pub signature: Option<String>,
}
pub struct LoginSuccess {
    pub uuid: UUID,
    pub username: String,
    pub properties_count: VarInt,
    properties: Vec<Property>,
}
impl Packet for LoginSuccess {
    fn read(_: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        unreachable!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut response = vec![];
        let packet_id = VarInt::from(0x02);
        packet_id.write(&mut response);
        self.uuid.write(&mut response);
        self.username.write(&mut response);
        self.properties_count.write(&mut response); // 0
        let mut real_reponse = Vec::new();
        let size = VarInt::from(response.len() as i32);
        size.write(&mut real_reponse);
        real_reponse.append(&mut response);
        real_reponse
    }
}
impl LoginSuccess {
    pub fn new(uuid: UUID, username: String) -> Self {
        Self {
            uuid,
            username,
            properties_count: VarInt::from(0),
            properties: Vec::new(),
        }
    }
}
