use crate::datatypes::datastructs::{String, VarInt, ByteArray, necesary::Necesary};

use crate::datatypes::packet_implementation::packets::Packet;
use crate::{TestNeccessaryTrait, DatastructError};

pub struct LoginEncriptionRequest {
    server_id: String,
    pub_key_length: VarInt,
    pub_key: ByteArray,
    verify_token_length: VarInt,
    verify_token: ByteArray,
}
impl Packet for LoginEncriptionRequest {
    fn read(_: &mut std::io::BufReader<&mut std::net::TcpStream>) -> Result<LoginEncriptionRequest, DatastructError> where Self: Sized {
        unreachable!("This function should never be called")
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        let mut data_size = 0;
        let packet_id = VarInt::new(0x01);
        let length = VarInt::new(
            packet_id.get_bytes() as i32 +
            self.server_id.get_value().len() as i32 +
            self.pub_key_length.get_bytes() as i32 +
            self.pub_key.get_value().len() as i32 +
            self.verify_token_length.get_bytes() as i32 +
            self.verify_token.get_value().len() as i32
        );
        length.write(&mut res);
        println!("length: {:?}", &res[data_size..]);
        data_size = res.len();
        packet_id.write(&mut res);
        println!("packet id: {:?}", &res[data_size..]);
        data_size = res.len();
        // self.server_id.write(&mut res);
        res.push(0x01);
        res.push(0x11);
        println!("server id data: {:?}", self.server_id.get_value());
        println!("server id: {:?}", &res[data_size..]);
        data_size = res.len();
        self.pub_key_length.write(&mut res);
        println!("pub key length: {:?}", &res[data_size..]);
        data_size = res.len();
        self.pub_key.write(&mut res);
        println!("pub key: {:?}", &res[data_size..]);
        data_size = res.len();
        self.verify_token_length.write(&mut res);
        println!("verify token length: {:?}", &res[data_size..]);
        data_size = res.len();
        self.verify_token.write(&mut res);
        println!("verify_token: {:?}", &res[data_size..]);
        res
    }

    fn read_length(_stream: &mut std::io::BufReader<&mut std::net::TcpStream>, _length: VarInt) -> Result<LoginEncriptionRequest, DatastructError> where Self: Sized {
        unreachable!()
    }
}
