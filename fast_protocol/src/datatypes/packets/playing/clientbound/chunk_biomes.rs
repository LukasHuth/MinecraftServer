use datatypes::{Array, VarInt};

use self::chunk_biomes_addions::BiomeData;

/// A list of all chunk biomes
pub struct ChunkBiomes {
    /// The number of chunks
    pub number_of_chunks: VarInt,
    /// A list of all chunk sections
    pub chunki_biome_data: Array<BiomeData>,
}
#[allow(missing_docs)]
pub mod chunk_biomes_addions {
    use binary_utils::{DataReader, DataWriter, ListDataReader};
    use datatypes::{ByteArray, Int, VarInt};

    pub struct BiomeData {
        pub chunk_z: Int,
        pub chunk_x: Int,
        pub size: VarInt,
        pub data: ByteArray,
    }
    impl DataWriter for BiomeData {
        async fn write(&self, writer: &mut (impl tokio::io::AsyncWrite + Unpin)) -> binary_utils::Result<()> {
            self.chunk_z.write(writer).await?;
            self.chunk_x.write(writer).await?;
            self.size.write(writer).await?;
            self.data.write(writer).await?;
            Ok(())
        }
    }
    impl DataReader for BiomeData {
        async fn read(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
            Ok(Self {
                chunk_z: Int::read(reader).await?,
                chunk_x: Int::read(reader).await?,
                size: VarInt::read(reader).await?,
                data: ByteArray::read_list(reader, 24).await?,
            })
        }
    }
}
