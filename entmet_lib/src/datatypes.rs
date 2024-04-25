use std::{ops::RangeBounds, slice::Iter};

use binary_utils::{DataWriter, DataReader, Error};
use datatypes::{Identifier, ImportantEnumTrait, ImportantFunctions};
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

// TODO: possibly implment things like rotation as type = ...

use crate::entities::entity::{CatVariant, FrogVariant, PaintingVariant};

/// A struct representing rotations in 3D space.
///
/// # Examples
///
/// ```
/// use entmet_lib::datatypes::Rotations;
///
/// let rotations = Rotations { x: 45.0, y: 30.0, z: 60.0 };
///
/// assert_eq!(rotations.x, 45.0);
/// assert_eq!(rotations.y, 30.0);
/// assert_eq!(rotations.z, 60.0);
/// ```
pub struct Rotations {
    /// The rotation around the x-axis
    pub x: f32, 
    /// The rotation around the y-axis
    pub y: f32, 
    /// The rotation around the z-axis
    pub z: f32
}
/// An enum representing directions
#[derive(Default)]
pub enum Direction {
    /// Downward direction
    #[default] Down,
    /// Upward direction
    Up,
    /// Northward direction
    North,
    /// Southward direction
    South,
    /// Westward direction
    West,
    /// Eastward direction
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
/// Module containing a particle enum
pub mod particles;
/// A generic struct representing a particle.
///
/// This struct contains data of type `T` and is constrained to types implementing both
/// the `DataReader` and `DataWriter` traits.
pub struct Particle<T> where T: DataReader + DataWriter {
    /// The type of particle
    pub particle_type: particles::ParticleType,
    /// The data associated with the particle
    pub data: Vec<T>,
}
/// A Module containing Structs used for Villager Data
pub mod villager_data;
/// A struct representing data associated with a villager.
///
/// # Examples
///
/// ```
/// use entmet_lib::datatypes::villager_data::{VillagerType, VillagerProfession};
/// use entmet_lib::datatypes::VillagerData;
///
/// let data = VillagerData {
///     villager_type: VillagerType::Plains,
///     villager_profession: VillagerProfession::Armorer,
///     level: 1,
/// };
/// assert_eq!(data.villager_type, VillagerType::Plains);
/// assert_eq!(data.villager_profession, VillagerProfession::Armorer);
/// assert_eq!(data.level, 1);
/// ```
pub struct VillagerData {
    /// The type of the villager
    pub villager_type: villager_data::VillagerType,
    /// The profession of the villager
    pub villager_profession: villager_data::VillagerProfession,
    /// The level of the villager
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
/// An enum representing different poses
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PoseEnum {
    /// Standing pose
    Standing = 0,
    /// Pose when falling and flying
    FallFlying = 1,
    /// Sleeping pose
    Sleeping = 2,
    /// Swimming pose
    Swimming = 3,
    /// Pose during a Spin attack
    SpinAttack = 4,
    /// Sneaking pose
    Sneaking = 5,
    /// Pose during a long jump
    LongJumping = 6,
    /// Dying pose
    Dying = 7,
    /// Pose when croaking
    Croaking = 8,
    /// Pose when using tongue???
    UsingTongue = 9,
    /// Sitting pose
    Sitting = 10,
    /// Raoring pose
    Roaring = 11,
    /// Pose when sniffing
    Sniffing = 12,
    /// Pose when emerging
    Emerging = 13,
    /// Pose during digging
    Digging = 14,
}
impl PoseEnum {
    /// function to get an iterator off all options
    ///
    /// # Returns
    ///
    /// Returns an Iter to a static array of all poses
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
/// A struct to describe a Global Position ( world and position )
pub struct GlobalPosition {
    /// world name
    pub identifier: datatypes::Identifier,
    /// Position in the named world
    pub position: datatypes::Position,
}
/// An enum with all Sniffer states
#[derive(Default)]
#[repr(u8)]
pub enum SnifferEnum {
    /// The Sniffer is idling.
    #[default] Idling = 0,
    /// The sniffer is feeling happy
    FeelingHappy = 1,
    /// The sniffer is scenting something
    Scenting = 2,
    /// The sniffer is sniffing for something
    Sniffing = 3,
    /// The sniffer is searching for something
    Searching = 4,
    /// The sniffer is digging
    Digging = 5,
    /// The sniffer is rising
    Rising = 6,
}
impl ImportantEnumTrait for SnifferEnum {
    fn new(_data: u64) -> binary_utils::Result<Self> {
        todo!()
    }
}
/// A Data Wrapper for an Vector in 3D space
pub struct Vector3 {
    /// The x-coordinate of the vector
    pub x: f32,
    /// The y-coordinate of the vector
    pub y: f32,
    /// The z-coordinate of the vector
    pub z: f32,
}
/// A Data Wrapper for a Quaternion.
pub struct Quaternion {
    /// The x-component of the quaternion.
    pub x: f32,
    /// The y-component of the quaternion.
    pub y: f32,
    /// The z-component of the quaternion.
    pub z: f32,
    /// The w-component of the quaternion.
    pub w: f32,
}
pub(crate) struct EntityMetadataField<T: DataReader + DataWriter> {
    pub id: u8,
    pub data: Option<EntityMetadataData<T>>,
}
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum EntityMetadataData<T: DataReader + DataWriter> {
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
/// An struct that can hold data in a specified range
pub struct Range<const S: u8, const E: u8>(pub u8);
impl<const S: u8, const E: u8> RangeBounds<u8> for Range<S, E> {
    fn start_bound(&self) -> std::ops::Bound<&u8> {
        std::ops::Bound::Included(&S)
    }

    fn end_bound(&self) -> std::ops::Bound<&u8> {
        std::ops::Bound::Included(&E)
    }
}
/// Trait to identify an blockstate
pub trait BlockStateIdentifier {}
/// Module containing all Block State types
pub mod block_states {
    use super::{Range, BlockStateIdentifier};
    /// Variant ranging from 0 to 1
    ///
    /// # Used for
    /// - Bamboo
    pub type AgeBamboo = Range<0, 1>;
    impl BlockStateIdentifier for AgeBamboo {}
    /// Variant ranging from 0 to 2
    ///
    /// # Used for
    /// - Cocoa
    pub type AgeCocoa = Range<0, 2>;
    impl BlockStateIdentifier for AgeCocoa {}
    /// Variant ranging from 0 to 3
    ///
    /// # Used for
    /// - Nether Wart
    /// - Beetroots
    /// - Frosted Ice
    /// - Sweet Berry Bush
    pub type AgeNWBFISBB = Range<0, 3>;
    impl BlockStateIdentifier for AgeNWBFISBB {}
    /// Variant ranging from 0 to 5
    ///
    /// # Used for
    /// - Chorus Flower
    pub type AgeChorusFlower = Range<0, 5>;
    impl BlockStateIdentifier for AgeChorusFlower {}
    /// Variant ranging from 0 to 7
    ///
    /// # Used for
    /// - Wheat Crops
    /// - Pumpkin Stem
    /// - Melon Stem
    /// - Carrots
    /// - Potatoes
    pub type AgeFood = Range<0, 7>;
    impl BlockStateIdentifier for AgeFood {}
    /// Variant ranging from 0 to 15
    ///
    /// # Used for
    /// - Fire
    /// - Cactus
    /// - Sugar Cane
    pub type AgeSpecial = Range<0, 15>;
    impl BlockStateIdentifier for AgeSpecial {}
    /// Variant ranging from 0 to 25
    ///
    /// # Used for
    /// - Kelp
    pub type AgeKelp = Range<0, 25>;
    impl BlockStateIdentifier for AgeKelp {}
    /// Variant allowing true or false
    ///
    /// # Used for
    /// - Tripwire Hook
    /// - Tripwire
    pub type Attached = bool;
    impl BlockStateIdentifier for Attached {}
    /// Variant allowing 4 options
    ///
    /// # Used for
    /// - Tripwire Hook
    /// - Tripwire
    pub type Attachent = super::AttachmentType;
    impl BlockStateIdentifier for Attachent {}
    /// Variant allowing 2 options
    ///
    /// # Used for
    /// - Nether Portal
    pub type Axis2D = super::Axis2DVariant;
    impl BlockStateIdentifier for Axis2D {}
    /// Variant allowing 3 options
    ///
    /// # Used for
    /// - Log
    /// - Stem
    /// - Basalt
    /// - Bone Block
    /// - Verdant Froglight
    /// - Muddy Mangrove Roots
    /// - Chain
    /// - Hay Bale
    /// - Purpur Pillar
    /// - Quartz Pillar
    /// - Deepslate
    pub type Axis3D = super::Axis3DVariant;
    impl BlockStateIdentifier for Axis3D {}
    // TODO: Add the missing Block states that follow this:
    // https://minecraft.fandom.com/wiki/Java_Edition_data_values#axis
}
/// Enum containing how a block can be attached
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AttachmentType {
    /// Attached at the ceiling
    Ceiling = 0,
    /// Attached at two walls
    DoubleWall = 1,
    /// Attached at the floor
    Floor = 2,
    /// Attached at one wall
    SingleWall = 3,
}
/// Enum containing how a block can be oriented in a 2D plane
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Axis2DVariant {
    /// The block is oriented towards X
    X = 0,
    /// The block is oriented towards Y
    Y = 1,
}
/// Enum containing how a block can be oriented in a 3D plane
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Axis3DVariant {
    /// The block is oriented towards X
    X = 0,
    /// The block is oriented towards Y
    Y = 1,
    /// The block is oriented towards Z
    Z = 2,
}
/// A struct for handling masks
#[derive(Debug, PartialEq, Clone)]
pub struct Mask<T> {
    mask: u8,
    options: Vec<T>
}
impl<T: Into<u8> + Copy + PartialEq> Mask<T> {
    /// function to create a new instance of `Mask`
    pub fn new() -> Self {
        Self::default()
    }
    /// function to add an option to the mask
    pub fn add(&mut self, option: T) {
        let mask: u8 = self.mask.into();
        let option_data: u8 = option.into();
        // Already masked because the adding failed
        if (mask | option_data) == mask { return }
        self.options.push(option);
        self.mask = mask | option_data;
        assert_eq!(self.mask.count_ones(), self.options.len() as u32);
    }
    /// function to remove an option from the mask
    pub fn remove(&mut self, option: T) {
        let mask: u8 = self.mask.into();
        let option_data: u8 = option.into();
        // Already not contained because the adding worked
        if (mask | option_data) != mask { return }
        if let Some(index) = self.options.iter().position(|e| *e == option) {
            self.options.remove(index);
            self.mask = mask & !option_data;
        }
        assert_eq!(self.mask.count_ones(), self.options.len() as u32);
    }
    /// function to get all the active options
    pub fn get_options(&self) -> &[T] {
        &self.options
    }
}
impl<T> Default for Mask<T>
    where T: Into<u8> + Copy + PartialEq {
    fn default() -> Self {
        Self { mask: 0u8, options: Vec::new() }
    }
}
