#![feature(buf_read_has_data_left)]
pub trait ImportantEnumTrait : Sized {
    fn new(data: u64) -> errors::Result<Self>;
}
pub mod datatypes;
pub mod utils;
pub mod errors;
