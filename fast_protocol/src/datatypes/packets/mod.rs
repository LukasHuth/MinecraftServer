use std::io::{Read, Seek, SeekFrom, BufRead};

use crate::utils::{DataReader, peek, consume_utf16be_char, consume_n_bytes};
use crate::errors::{Result, Error};

use super::datatype_definition::VarInt;

mod handshake;
mod login;
mod status;
mod configuration;
mod playing;
pub use handshake::*;
pub use login::*;
pub use status::*;
pub use configuration::*;
pub use playing::*;
trait PacketEnum: Sized {
    fn new(data: VarInt) -> Result<Self>;
}
pub enum State {
    Handshake,
    Login,
    Status,
    Configuration,
    Playing,
}
pub enum ServerboundPackets {
    None,
    LegacyPing,
}
impl ServerboundPackets {
    pub fn read(reader: &mut (impl Read + BufRead), state: &mut State) -> Result<Self> {
        match reader.has_data_left() {
            Ok(v) => match v {
                false => return Ok(ServerboundPackets::None),
                true => (),
            },
            Err(_) => return Error::InvalidStructure.into(),
        }
        match state {
            State::Handshake => Self::handshake(reader),
            State::Status => Self::status(reader),
            State::Login => Self::login(reader),
            State::Configuration => Self::configuration(reader),
            State::Playing => Self::playing(reader),
        }
    }
    fn handshake(reader: &mut (impl Read + BufRead)) -> Result<Self> {
        let peeked_data = peek(reader)?;
        if peeked_data[0] == 0xFE && peeked_data[1] == 0x01 {
            let mut data = [0; 5];
            match reader.read_exact(&mut data) { Ok(_) => Ok(()), Err(_) => Error::NotEnoughtBytes.into()}?;
            let length = ((data[3] as u16) << 8) | data[4] as u16;
            for _ in 0..length {
                consume_utf16be_char(reader)?;
            }
            let mut length = [0;2];
            match reader.read_exact(&mut length) { Ok(_) => Ok(()), Err(_) => Error::NotEnoughtBytes.into()}?;
            let length = ((length[0] as u16) << 8) | length[1] as u16;
            consume_n_bytes(reader, length as u64)?;
            Ok(Self::LegacyPing)
        } else {
            todo!()
        }
    }
    fn status(_reader: &mut impl Read) -> Result<Self> {
        todo!()
    }
    fn login(_reader: &mut impl Read) -> Result<Self> {
        todo!()
    }
    fn configuration(_reader: &mut impl Read) -> Result<Self> {
        todo!()
    }
    fn playing(_reader: &mut impl Read) -> Result<Self> {
        todo!()
    }
}
