use std::slice::Iter;

use binary_utils::{DataWriter, DataReader, Error};
use datatypes::{Enum, Identifier, ImportantEnumTrait, ImportantFunctions};
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::entities::entity::{living_entity::mob::pathfinder_mob::ageable_mob::animal::{frog::FrogVariant, tameable_animal::cat::CatVariant}, painting::PaintingVariant};

pub struct Rotations {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Default)]
pub enum Direction {
    #[default] Down,
    Up,
    North,
    South,
    West,
    East,
}
impl ImportantEnumTrait for Direction {
    fn new(data: u64) -> binary_utils::Result<Self> {
        match data {
            0 => Ok(Self::Down),
            1 => Ok(Self::Up),
            2 => Ok(Self::North),
            3 => Ok(Self::South),
            4 => Ok(Self::West),
            5 => Ok(Self::East),
            6..=u64::MAX => Err(Error::InvalidStructure),
        }
    }
}
pub struct Optional<T> where T: DataReader + DataWriter {
    pub present: datatypes::Boolean,
    pub data: Option<T>
}
pub mod particles;
pub struct Particle<T> where T: DataReader + DataWriter {
    pub particle_type: particles::ParticleType,
    pub data: Vec<T>,
}
pub mod villager_data;
pub struct VillagerData {
    pub villager_type: villager_data::VillagerType,
    pub villager_profession: villager_data::VillagerProfession,
    pub level: i32,
}
impl Default for VillagerData {
    fn default() -> Self {
        Self {
            villager_type: villager_data::VillagerType::Plains,
            villager_profession: villager_data::VillagerProfession::None,
            level: 1
        }
    }
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PoseEnum {
    Standing = 0,
    FallFlying = 1,
    Sleeping = 2,
    Swimming = 3,
    SpinAttack = 4,
    Sneaking = 5,
    LongJumping = 6,
    Dying = 7,
    Croaking = 8,
    UsingTongue = 9,
    Sitting = 10,
    Roaring = 11,
    Sniffing = 12,
    Emerging = 13,
    Digging = 14,
}
impl PoseEnum {
    pub fn iterator() -> Iter<'static, Self> {
        use PoseEnum::*;
        static POSES: [PoseEnum; 15] = [Standing, FallFlying, Sleeping, Swimming, SpinAttack, Sneaking, LongJumping, Dying, Croaking, UsingTongue, Sitting, Roaring, Sniffing, Emerging, Digging];
        POSES.iter()
    }
}
impl ImportantEnumTrait for PoseEnum {
    fn new(data: u64) -> binary_utils::Result<Self> {
        if data >= PoseEnum::iterator().len() as u64 { return Err(binary_utils::Error::InvalidStructure); }
        match PoseEnum::iterator().map(|&s|(s, s as u8)).filter(|(_, id)| *id == data as u8).next() {
            Some(v) => Ok(v.0),
            None => Err(binary_utils::Error::InvalidStructure),
        }
    }
}
pub struct Pose {
    pub pose: Enum<PoseEnum, datatypes::VarInt>,
}
pub struct GlobalPosition {
    pub identifier: datatypes::Identifier,
    pub position: datatypes::Position,
}
#[derive(Default)]
#[repr(u8)]
pub enum SnifferEnum {
    #[default] Idling = 0,
    FeelingHappy = 1,
    Scenting = 2,
    Sniffing = 3,
    Searching = 4,
    Digging = 5,
    Rising = 6,
}
impl ImportantEnumTrait for SnifferEnum {
    fn new(_data: u64) -> binary_utils::Result<Self> {
        todo!()
    }
}
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub enum MinecraftColor {
    White = 0,
    Orange = 1,
    Magenta = 2,
    LightBlue = 3,
    Yellow = 4,
    Lime = 5,
    Pink = 6,
    Gray = 7,
    LightGray = 8,
    Cyan = 9,
    Purple = 10,
    Blue = 11,
    Brown = 12,
    Green = 13,
    #[default] Red = 14,
    Black = 15,
}
pub struct EntityMetadataField<T: DataReader + DataWriter> {
    pub id: u8,
    pub data: Option<EntityMetadataData<T>>,
}
#[repr(u8)]
pub enum EntityMetadataData<T: DataReader + DataWriter> {
    Byte(i8) = 0,
    VarInt(i32) = 1,
    VarLong(i64) = 2,
    Float(f32) = 3,
    String(String) = 4,
    TextComponent(nbt_lib::datatypes::TextComponent) = 5,
    OptionalTextComponent(Option<nbt_lib::datatypes::TextComponent>) = 6,
    Slot(slot_lib::Slot) = 7,
    Boolean(bool) = 8,
    Rotation(f32, f32, f32) = 9,
    Position(i32, i32, i16) = 10,
    OptionalPosition(Option<(i32, i32, i16)>) = 11,
    Direction(Direction) = 12,
    OptionalUUID(Option<u128>) = 13,
    BlockState(i32) = 14,
    OptionalBlockState(Option<i32>) = 15,
    NBT(nbt_lib::NbtValue) = 16,
    Particle(Particle<T>) = 17,
    VillagerData(VillagerData) = 18,
    OptionalVarint(Option<i32>) = 19,
    Pose(PoseEnum) = 20,
    CatVariant(CatVariant) = 21,
    FrogVariant(FrogVariant) = 22,
    OptionalGlobalPosition(Option<(Identifier, (i32, i32, i16))>) = 23,
    PaintingVariant(PaintingVariant) = 24,
    SnifferState(SnifferEnum) = 25,
    Vector3(f32, f32, f32) = 26,
    Quaternion(f32, f32, f32, f32) = 27,
}
impl<T: DataReader + DataWriter> EntityMetadataData<T> {
    pub fn to_u8(&self) -> u8 {
        match self {
            EntityMetadataData::Byte(_) => 0,
            EntityMetadataData::VarInt(_) => 1,
            EntityMetadataData::VarLong(_) => 2,
            EntityMetadataData::Float(_) => 3,
            EntityMetadataData::String(_) => 4,
            EntityMetadataData::TextComponent(_) => 5,
            EntityMetadataData::OptionalTextComponent(_) => 6,
            EntityMetadataData::Slot(_) => 7,
            EntityMetadataData::Boolean(_) => 8,
            EntityMetadataData::Rotation(_, _, _) => 9,
            EntityMetadataData::Position(_, _, _) => 10,
            EntityMetadataData::OptionalPosition(_) => 11,
            EntityMetadataData::Direction(_) => 12,
            EntityMetadataData::OptionalUUID(_) => 13,
            EntityMetadataData::BlockState(_) => 14,
            EntityMetadataData::OptionalBlockState(_) => 15,
            EntityMetadataData::NBT(_) => 16,
            EntityMetadataData::Particle(_) => 17,
            EntityMetadataData::VillagerData(_) => 18,
            EntityMetadataData::OptionalVarint(_) => 19,
            EntityMetadataData::Pose(_) => 20,
            EntityMetadataData::CatVariant(_) => 21,
            EntityMetadataData::FrogVariant(_) => 22,
            EntityMetadataData::OptionalGlobalPosition(_) => 23,
            EntityMetadataData::PaintingVariant(_) => 24,
            EntityMetadataData::SnifferState(_) => 25,
            EntityMetadataData::Vector3(_, _, _) => 26,
            EntityMetadataData::Quaternion(_, _, _, _) => 27,
        }
    }
}
impl<T> DataWriter for EntityMetadataField<T> where T: DataReader + DataWriter {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut buf_writer = BufWriter::new(writer);
        datatypes::UnsignedByte::new(self.id).write(&mut buf_writer).await?;
        if let Some(data) = self.data.as_ref() {
            data.write(&mut buf_writer).await?;
        }
        if let Err(_) = buf_writer.flush().await {
        }
        Ok(())
    }
}
impl<T> DataWriter for EntityMetadataData<T> where T: DataReader + DataWriter {
    async fn write(&self, writer: &mut (impl AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        let mut buf_writer = BufWriter::new(writer);
        datatypes::UnsignedByte::new(self.to_u8()).write(&mut buf_writer).await?;
        match self {
            Self::Byte(d) => datatypes::Byte::new(*d).write(&mut buf_writer).await?,
            _ => todo!(),
        }
        if let Err(_) = buf_writer.flush().await {
        }
        Ok(())
    }
}
