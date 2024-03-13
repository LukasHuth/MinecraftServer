use crate::datatypes::datastructs::{VarInt, ByteArray};

pub struct LoginEncriptionResponse {
    pub shared_secret_length: VarInt,
    pub shared_secret: ByteArray,
    pub verify_token_length: VarInt,
    pub verify_token: ByteArray,
}
