use std::io;

use nbt_lib::{traits::NbtRead, version::Java};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let uncompress: bool = args.iter().any(is_uncompress_str);
    let args: Vec<String> = args.iter().map(|e|e.clone()).filter(is_not_uncompress_str).map(|e|e.clone()).collect();
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Please specify a file to open"));
    }
    let filename = args[1].clone();
    let file_content = std::fs::read(filename)?;
    let reader = if uncompress {
        nbt_lib::reader::NbtReader::from_compressed_data(file_content)
    } else {
        nbt_lib::reader::NbtReader::new(file_content)
    };
    let res = Java::from_reader(reader);
    match res {
        Ok(v) => Ok(println!("Result: {:?}", v)),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", e)))
    }
}
#[inline]
fn is_uncompress_str(e: &String) -> bool {
    e == "--uncompress"
}
#[inline]
fn is_not_uncompress_str(e: &String) -> bool {
    !is_uncompress_str(e)
}
