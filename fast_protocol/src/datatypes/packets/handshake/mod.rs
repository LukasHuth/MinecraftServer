use crate::{datatypes::{json_datastructures::{StatusResponseJSON, Player, Version}, datatype_definition::{VarInt, Long}}, utils::{DataWriter, write_bytes}};

const IMAGE: &str = "iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAB/lBMVEUdtTwctDsdtDwctTsdtDsctTwcszsdtjweuj0duDwcrzoYmjUYmzUYmTUapTgYmjYFNx8GOyAGOiAGOh8GPCAEMR0PaysevT0evT4YmzYGOx8HPyEHPiEHQSIPbisduj0HPSIGOh4HPB8FMRwPayoQaioGPSEBCAQAAwEABAIBBwMAAAANSxofwUAcsjsfwT8NTBoBCQUAAgANSBkNSRkABQIAAQAABQEevz8csDoNShoAAwINSxkgykIeujwFMR0PaioNSBgMRBcQYSMUiDIUgzAUhDAUiTIgy0IViTIDKxwFNR4DKxsViDIeuTwUhTEFOiEHRCMHQiIFOiAevj4UgC4EKBgGMxsGMRoUgC8csToewD8TbCQTbCUdtzwfw0ATbiUcrjobrDkbrTkbqjgduT0SaCMSaCQNXSYHQCEIRyIIRiIIRCEJSiQFKhUGKhUNXSceuDwMVSUFNR8GPyIEIxMFIxMLVSYduD0MWCYGOiEHQCIIRCQFJhQHRCQMWSceuT0JMhQBBgUCEAgCDgcCDwgCCAUABgUJMhMeuT4HJw4HKA8AAgEHKA4HJw8DEAUIKg8HJw0EEAYgx0IOSxoMSBkIKQ8ABAENShkHKQ4HJg4NRxgNRxkWfSsTaiQTbiYTbSUXii8YijATaCQfwj8fwkAevD4dvD4dtTv////Ggx04AAAAAWJLR0SpJw8GBAAAAAd0SU1FB+gDDBUFA8UmMkMAAAMMSURBVFjD7Zf5VxJRGIbnDsuACA0MiMZSoiUyLAWEkEsuWbZpFmlp4YJkbm3aaraJlbZpVpaltmj1Z3aZRW7K6Aid0/r85Hnv9fF1DnO/C4YBLH1wCZ7R70OBFAMZNZBlKsi4wX/BzxbAn+UIIPkZA0IrKxoAQoGgRKoBJbpCgNQNAJGlylZxZKs1y/tkgNCokyuqLCJ1A7liE6nVUSx6Q45RjnPb5Lk5Bj23oNOSKoVcQJC32WQ2sVhIKyqwkhZuwWzS5m0RFFDmrVQ+g62gMJcXSOXGbQU2Nqe2m3V5wg0oE1VkL07goJ0uNy9we3bQDia274Rb1hTkF9MMXtq3LMDdLh8MGIrFCuy0HxX4YcCwK80G/g03+GMFeovNEbB7ISW0Pxji355QcDddkkjtAYfNohf+IKkNZEEp36CsvMLIUVFexjco3UMa1AINgFJTaK10Vvl9EH91zd7afRy1+2uq2bTKWWkt1CgFX2ejMdfFEqw7cJA8xEEePlIXZHMPLCT4OjPHhpslVN9wtPEYR+PxhvoQt7DWgQIDBinEHT7R1HySo7npVNidSNl1ZH+qMxGOS0hL62n+uSWe6JnWlkSK4Sv2phLIcCISiYC2dlTQ3gZgSKz6YykEAJd0dEajXbGzqKA71hWNdnZIcMn6AhA519Pb1z9wng7wggB9YaC/r7fnonLF7tSCS5cH9Vpq6AqNcHWI0uoHr10XJVDeuGkZNt0aQQUjt03DFt0dcYK79+6PxuNjRajgwVg8Pvrw0bgowfhE8r9HCdATCnGCx8nnj/KEfipS8CzTBn+PwBtYxvtLGjyfnJrgmJp8sWGBl3ZOR/grSWT6JQw2KvCxZwh7urxKR+By8wJ2sKQh+HEy/VOC1/QMHK5vUEFiuM7Qb0UKZpkG75BLlus902BWjEAKiLn5hQ8L3R9j/DVPJo99+gyj+TkCSNcVwA7hRY/Hs/QFvSt/XYLRYljMYMEBxk1R9LbOzVw4dkQ0wL6xoJFsdbSGQDy/iQDPhIRAIksfCfzuDG80GTSQ4t8BoLsUlayVyQIAAAAldEVYdGRhdGU6Y3JlYXRlADIwMjQtMDMtMTJUMjE6MDQ6NDArMDA6MDBCNC7TAAAAJXRFWHRkYXRlOm1vZGlmeQAyMDI0LTAzLTEyVDIxOjA0OjQwKzAwOjAwM2mWbwAAAABJRU5ErkJggg==";

pub struct StatusResponsePacket(StatusResponseJSON);
pub struct PongPacket(Long);
pub struct LegacyPongPacket(std::string::String, std::string::String, u16, u16);
impl StatusResponsePacket {
    pub fn new(version: std::string::String, protocol: u16, max_players: u16, player_count: u16, players: Vec<Player>, modt: std::string::String) -> Self {
        Self(StatusResponseJSON::new(version, protocol, max_players, player_count, players, modt, format!("data:image/png;base64,{}", IMAGE)))
    }
}
impl PongPacket {
    pub fn new(data: i64) -> Self {
        Self(Long::new(data))
    }
}
impl LegacyPongPacket {
    pub fn new(server_version: std::string::String, motd: std::string::String, current_players: u16, max_players: u16) -> Self {
        Self(server_version, motd, current_players, max_players)
    }
}
impl DataWriter for StatusResponsePacket {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x00);
        id.write(&mut data)?;
        self.0.write(&mut data)?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
impl DataWriter for PongPacket {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let mut data = Vec::new();
        let id = VarInt::new(0x01);
        id.write(&mut data)?;
        self.0.write(&mut data)?;
        let length = VarInt::new(data.len() as i32);
        length.write(writer)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
impl DataWriter for LegacyPongPacket {
    fn write(&self, writer: &mut impl std::io::prelude::Write) -> crate::errors::Result<()> {
        let data = format!("§1\0{}\0{}\0{}\0{}\0{}", 127, self.0, self.1, self.2, self.3);
        let data = data.as_bytes().to_vec();
        let data: Vec<u8> = data.iter().map(|v|[0, *v]).flatten().collect();
        let length = data.len();
        let length = [(length << 8) as u8, length as u8];
        write_bytes(writer, &[0xFF])?;
        write_bytes(writer, &length)?;
        write_bytes(writer, &data)?;
        Ok(())
    }
}
