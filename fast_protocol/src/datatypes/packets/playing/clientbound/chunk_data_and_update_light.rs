use binary_utils::{write_bytes, DataWriter};
use datatypes::{Array, BitSet, ByteArray, ImportantFunctions as _, Int, TypedImportantFunctions as _, VarInt};
use nbt_lib::{
    create_compound_map, datatypes::NBT, traits::AsNbtValue, version::JavaNetty, NbtValue,
};
use tokio::io::{AsyncWriteExt, BufWriter};

/// Packet to send a chunk to the player
pub struct ChunkDataAndUpdateLight {
    /// The x coordinate from the chunk
    pub chunk_x: Int,
    /// The z coordinate from the chunk
    pub chunk_z: Int,
    /// The Heightmaps data source: <https://wiki.vg/Chunk_Format#Heightmaps_structure>
    pub heightmaps: chunk_data_and_update_light_extra::Heightmaps,
    /// See: <https://wiki.vg/Chunk_Format#Data_structure>
    /// Convert to NBT bytes when sending
    pub data: Vec<level_lib::anvil::region::chunk::section::ChunkSection>,
    /// all block entities of the chunk
    pub block_entities: Array<chunk_data_and_update_light_extra::BlockEntityData>,
    /// each bit represents whether data is available in the skylight array for the corrosponding
    /// section
    pub sky_light_mask: [bool; 26],
    /// each bit represents whether data is available in the block light array for the
    /// corrosponding section
    pub block_light_mask: [bool; 26],
    /// each bit represents whether the corrosponding data is full of zeros
    pub empty_sky_light_mask: [bool; 26],
    /// each bit represents whether the corrosponding data is full of zeros
    pub empty_block_light_mask: [bool; 26],
    /// The sky light array
    pub sky_light_arrays: Array<chunk_data_and_update_light_extra::SkyLight>,
    /// The length of the block light
    pub block_light_array_count: VarInt,
    /// The block light array
    pub block_light_arrays: Array<chunk_data_and_update_light_extra::BlockLight>,
}
#[allow(missing_docs)]
pub mod chunk_data_and_update_light_extra {
    use binary_utils::{DataReader, DataWriter};
    use datatypes::{ByteArray, ImportantFunctions, Short, UnsignedByte, VarInt};
    use nbt_lib::{create_compound_map, datatypes::NBT, traits::AsNbtValue, NbtValue};

    pub struct Heightmaps {
        pub motion_blocking: [u16; 16 * 16],
        pub world_surface: [u16; 16 * 16],
    }
    impl AsNbtValue for Heightmaps {
        fn as_nbt_value(&self) -> Result<nbt_lib::NbtValue, ()> {
            let heightmap_d0 = self
                .motion_blocking
                .chunks(64 / 9)
                .map(|elements| {
                    let mut result = 0;
                    let mut used_bytes = 0;
                    for element in elements {
                        result |= (*element as i64 & 0x1F) << used_bytes;
                        used_bytes += 9;
                    }
                    result
                })
                .collect();
            let heightmap_d1 = self
                .world_surface
                .chunks(64 / 9)
                .map(|elements| {
                    let mut result = 0;
                    let mut used_bytes = 0;
                    for element in elements {
                        result |= (*element as i64 & 0x1F) << used_bytes;
                        used_bytes += 9;
                    }
                    result
                })
                .collect();
            let heightmaps = NbtValue::Compound(
                None,
                create_compound_map!(
                    MOTION_BLOCKING: NbtValue::LongArray(heightmap_d0),
                    WORLD_SURFACE: NbtValue::LongArray(heightmap_d1)
                ),
            );
            Ok(heightmaps)
        }
    }
    pub struct BlockEntityData {
        pub x: u8,
        pub z: u8,
        pub y: Short,
        pub type_id: VarInt,
        pub data: NBT,
    }
    impl DataWriter for BlockEntityData {
        async fn write(
            &self,
            writer: &mut (impl tokio::io::AsyncWrite + Unpin),
        ) -> binary_utils::Result<()> {
            let packet_xz = ((self.x & 15) << 4) | (self.z & 15);
            UnsignedByte::new(packet_xz).write(writer).await?;
            self.y.write(writer).await?;
            self.type_id.write(writer).await?;
            self.data.write(writer).await?;
            Ok(())
        }
    }
    impl DataReader for BlockEntityData {
        async fn read(
            _reader: &mut (impl tokio::io::AsyncRead + Unpin),
        ) -> binary_utils::Result<Self> {
            unreachable!()
        }
    }
    pub struct SkyLight {
        pub sky_light: [u8; 4096],
    }
    impl DataReader for SkyLight {
        async fn read(
            _reader: &mut (impl tokio::io::AsyncRead + Unpin),
        ) -> binary_utils::Result<Self> {
            unreachable!()
        }
    }
    impl DataWriter for SkyLight {
        async fn write(
            &self,
            writer: &mut (impl tokio::io::AsyncWrite + Unpin),
        ) -> binary_utils::Result<()> {
            VarInt::new(2048).write(writer).await?;
            ByteArray::new(
                self.sky_light
                    .chunks(2)
                    .map(|e| ((e[0] & 15) << 4) | (e[1] & 15))
                    .collect(),
            )
            .write(writer)
            .await?;
            Ok(())
        }
    }
    pub struct BlockLight {
        pub block_light: [u8; 4096],
    }
    impl DataReader for BlockLight {
        async fn read(
            _reader: &mut (impl tokio::io::AsyncRead + Unpin),
        ) -> binary_utils::Result<Self> {
            unreachable!()
        }
    }
    impl DataWriter for BlockLight {
        async fn write(
            &self,
            writer: &mut (impl tokio::io::AsyncWrite + Unpin),
        ) -> binary_utils::Result<()> {
            VarInt::new(2048).write(writer).await?;
            ByteArray::new(
                self.block_light
                    .chunks(2)
                    .map(|e| ((e[0] & 15) << 4) | (e[1] & 15))
                    .collect(),
            )
            .write(writer)
            .await?;
            Ok(())
        }
    }
}
