use binary_utils::{Result, write_bytes, ListDataReader, Error, read_byte};
use tokio::io::{AsyncWrite, AsyncRead, AsyncReadExt as _, AsyncWriteExt as _};
use crate::ImportantFunctions;
use super::*;

impl DataWriter for UUID {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        write_bytes(writer, &self.0.to_be_bytes()).await?;
        Ok(())
    }
}
impl DataReader for UUID {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        use crate::Long;
        let l0 = Long::read(reader).await?.get_value();
        let l1 = Long::read(reader).await?.get_value();
        let data = ((l0 as u128) << 64) | l1 as u128;
        Ok(Self(data))
    }
}
impl GetU64 for VarInt {
    fn get_u64(&self) -> u64 {
        self.0 as u64
    }
}
impl<T> ListDataReader for Array<T> where T: DataReader + DataWriter{
    async fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> Result<Self> where Self: Sized {
        let mut data = Vec::new();
        for _ in 0..length {
            data.push(T::read(reader).await?);
        }
        Ok(Self(data))
    }
}
impl<T> DataWriter for Array<T> where T: DataReader + DataWriter{
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        for e in self.0.iter() {
            e.write(writer).await?;
        }
        Ok(())
    }
}
impl ListDataReader for ByteArray {
    async fn read_list(reader: &mut (impl AsyncRead + Unpin), length: usize) -> Result<Self> where Self: Sized {
        let mut data = vec![0; length];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into(),
        }
    }
}
impl DataWriter for ByteArray {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        match writer.write_all(&self.0).await {
            Ok(_) => Ok(()),
            Err(_) => Error::FailedToWrite.into(),
        }
    }
}
impl<const S: usize> DataReader for FixedBitSet<S> {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let mut data = [0u8; S];
        match reader.read_exact(&mut data).await {
            Ok(_) => Ok(Self(data)),
            Err(_) => Error::NotEnoughtBytes(format!("{}:{}", file!(), line!())).into(),
        }
    }
}
impl DataReader for VarInt {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let mut data: i32 = 0;
        loop {
            let current = read_byte(reader, line!(), file!()).await?;
            data <<= 7;
            data |= (current & 0x7F) as i32;
            if current < 0x80 { break; }
        }
        Ok(Self(data))
    }
}
impl DataWriter for VarInt {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
        let mut data = self.0;
        loop {
            let mut current = ((data & 0x7F) as u8) | 0x80;
            data >>= 7;
            if data == 0 {
                current &= 0x7F;
                return write_bytes(writer, &[current]).await;
            }
            write_bytes(writer, &[current]).await?;
        }
    }
}
impl<T, S> DataReader for Enum<T, S> where S: DataReader + GetU64, T: ImportantEnumTrait {
    async fn read(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> where Self: Sized {
        let original_value = S::read(reader).await?;
        let value = T::new(original_value.get_u64())?;
        Ok(Self(value, original_value))
    }
}
