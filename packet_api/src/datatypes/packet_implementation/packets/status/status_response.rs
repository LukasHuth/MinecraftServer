use serde::{Serialize, Deserialize};

use crate::datatypes::datastructs::{player::Player, VarInt, necesary::Necesary};

use crate::datatypes::packet_implementation::packets::Packet;
use crate::TestNeccessaryTrait;

const IMAGE_BASE64: &str = "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";

#[derive(Serialize, Deserialize)]
pub struct StatusResponseVersion {
    pub name: String,
    pub protocol: u16,
}
#[derive(Serialize, Deserialize)]
pub struct StatusResponsePlayer {
    pub name: String,
    pub id: String,
}
#[derive(Serialize, Deserialize)]
pub struct StatusResponsePlayers {
    pub max: u16,
    pub online: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<StatusResponsePlayer>>,
}
#[derive(Serialize, Deserialize)]
pub struct StatusResponseDescription {
    pub text: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: StatusResponseDescription,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub enforcesSecureChar: bool,
    pub previewsChar: bool,
}

impl Packet for StatusResponse {
    fn read(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Option<Self> where Self: Sized {
        unreachable!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        let json_str = serde_json::to_string(self).expect("Failed to convert data to json string").to_string();
        let packet_id = VarInt::new(0);
        let json_length = VarInt::new(json_str.len() as i32);
        let packet_length = VarInt::new(json_str.len() as i32 + json_length.get_bytes() as i32 + packet_id.get_bytes() as i32);
        packet_length.write(&mut result);
        packet_id.write(&mut result);
        json_length.write(&mut result);
        let mut json_bytes = json_str.as_bytes().to_vec();
        result.append(&mut json_bytes);
        result
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: VarInt) -> Option<Self> where Self: Sized {
        unreachable!()
    }
}
impl StatusResponse {
    pub fn new(version_name: String, protocol_version: u16, max_players: u16, players: Vec<Player>, server_description: std::string::String) -> Self {
        let sample = if players.len() == 0 { None } else {
            Some(players.iter().map(|p|StatusResponsePlayer { name: p.username.clone(), id: p.uuid.to_string() }).collect())
        };
        let version = StatusResponseVersion {
            name: version_name, protocol: protocol_version,
        };
        let players = StatusResponsePlayers {
            max: max_players,
            online: players.len() as u16,
            sample 
        };
        let description = StatusResponseDescription {
            text: server_description,
        };
        let favicon = Some(format!("data:image/png;base64,{}", IMAGE_BASE64));
        // let favicon = None;
        Self {
            version,
            players,
            description,
            favicon,
            enforcesSecureChar: true,
            previewsChar: true,
        }
    }
}
