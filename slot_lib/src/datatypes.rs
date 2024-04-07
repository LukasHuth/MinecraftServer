use binary_utils::{DataWriter, write_bytes, Error};
use datatypes::ImportantFunctions;
use nbt_lib::{version::JavaNetty, traits::NbtWrite};
use tokio::io::AsyncWrite;

use crate::Slot;

impl DataWriter for Slot {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        match self {
            Self::Empty => datatypes::Boolean::new(false).write(writer).await,
            Self::Data(id, count, nbt) => {
                let mut data = Vec::new();
                datatypes::Boolean::new(true).write(&mut data).await?;
                id.write(&mut data).await?;
                count.write(&mut data).await?;
                if let Some(nbt) = nbt {
                    if let Err(_) = JavaNetty::write_to(&nbt.get_value(), &mut data) {
                        return Err(Error::FailedToWrite);
                    }
                } else {
                    write_bytes(&mut data, &[0]).await?;
                }
                write_bytes(writer, &data).await
            }
        }
    }
}
impl ImportantFunctions for Slot {
    type InputType = (Option<i32>, Option<i8>, Option<nbt_lib::NbtValue>);
    type ReturnType = Self::InputType;

    fn new(data: Self::InputType) -> Self {
        if data.0.is_some() && data.1.is_some() {
            return Self::Data(datatypes::VarInt::new(data.0.unwrap_or(0)), datatypes::Byte::new(data.1.unwrap_or(0)), data.2.map(|d|nbt_lib::datatypes::NBT::new(d)));
        }
        Self::Empty
    }

    fn get_value(&self) -> Self::ReturnType {
        todo!()
    }
}
