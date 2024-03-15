use crate::datatypes::datastructs::{String, VarInt, UUID, Boolean, necesary::Necesary};

use crate::datatypes::packet_implementation::packets::Packet;
use crate::TestNeccessaryTrait;

const SKIN: &str = "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmM5NmExNGRjMWNiOTQzYjhmZjNjOTJhYWNiMDEwMmMyMzg5ZWVkZWY1MGQzNmI3MTRkMGRiOThiMjdhIn19fQ";

#[derive(Clone)]
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
        let packet_id = VarInt::new(0x02);
        packet_id.write(&mut response);
        self.uuid.write(&mut response);
        self.username.write(&mut response);
        self.properties_count.write(&mut response); // 0
        for prop in self.properties.clone() {
            prop.name.write(&mut response);
            prop.value.write(&mut response);
            prop.is_signed.write(&mut response);
            if *prop.is_signed.get_value() { prop.signature.unwrap().write(&mut response); }
        }
        let mut real_reponse = Vec::new();
        let size = VarInt::new(response.len() as i32);
        size.write(&mut real_reponse);
        real_reponse.append(&mut response);
        real_reponse
    }
}
impl LoginSuccess {
    pub fn new(uuid: UUID, username: String) -> Self {
        let property = Property {
            name: String::new(std::string::String::from("texture")),
            // value: String::new(std::string::String::from(SKIN)),
            value: String::new(format!("data:image/png;base64,{}", SKIN)),
            is_signed: Boolean::new(false),
            signature: None,
        };
        Self {
            uuid,
            username,
            properties_count: VarInt::new(1),
            properties: vec![property],
        }
    }
}
