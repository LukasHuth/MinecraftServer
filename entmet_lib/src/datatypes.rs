use std::slice::Iter;

use binary_utils::{DataWriter, DataReader, Error};
use datatypes::{ImportantEnumTrait, Enum, ImportantFunctions};

pub struct Rotations {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub enum Direction {
    Down,
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
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum CatVariantEnum {
    White = 0,
    Tuxedo = 1,
    Ginger = 2,
    Siamese = 3,
    BritishShorthair = 4,
    Calico = 5,
    Persian = 6,
    Ragdoll = 7,
    Tabby = 8,
    Black = 9,
    Jellie = 10,
}
impl Default for CatVariantEnum {
    fn default() -> Self {
        Self::Black
    }
}
impl ImportantFunctions for CatVariantEnum {
    type InputType = i32;

    type ReturnType = i32;

    fn new(data: Self::InputType) -> Self {
        if data > 10 || data < 0 { return Self::default() }
        match data {
            0 => Self::White,
            1 => Self::Tuxedo,
            2 => Self::Ginger,
            3 => Self::Siamese,
            4 => Self::BritishShorthair,
            5 => Self::Calico,
            6 => Self::Persian,
            7 => Self::Ragdoll,
            8 => Self::Tabby,
            9 => Self::Black,
            10 => Self::Jellie,
            _ => unreachable!()
        }
    }

    fn get_value(&self) -> Self::ReturnType {
        *self as u8 as i32
    }
}
pub struct CatVariant {
    pub variant: CatVariantEnum,
}
// TODO: frog variant
pub enum FrogEnum {
}
pub struct FrogVariant {
    pub variant: FrogEnum,
}
pub struct GlobalPosition {
    pub identifier: datatypes::Identifier,
    pub position: datatypes::Position,
}
// TODO:
#[repr(u8)]
pub enum SnifferEnum {
    Idling = 0,
    FeelingHappy = 1,
    Scenting = 2,
    Sniffing = 3,
    Searching = 4,
    Digging = 5,
    Rising = 6,
}
impl Default for SnifferEnum {
    fn default() -> Self {
        Self::Idling
    }
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
