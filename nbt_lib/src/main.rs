use std::io;

use nbt_lib::{traits::NbtRead, version::Java};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Please specify a file to open"));
    }
    let filename = args[1].clone();
    let file_content = std::fs::read(filename)?;
    let reader = nbt_lib::reader::NbtReader::from_compressed_data(file_content);
    let res = Java::from_reader(reader);
    match res {
        Ok(v) => Ok(println!("Result: {:?}", v)),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", e)))
    }
}
