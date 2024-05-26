use std::ops::{Deref, DerefMut};

use nbt_lib::{traits::{AsNbtValue, FromNbtValue}, NbtValue};

use nbt_lib::{unwrap_to_empty, unwrap_to_empty_if_exists};


/// Definition of the layout of an heightmap
///
/// # Note
///
/// Every element(u64) stores 7 values each of them 9-bits long
pub type Heightmap = [u64;37];
impl HeightMapIdentifier for Heightmap {
    fn get_default() -> Self {
        [0;37]
    }
}
trait HeightMapIdentifier {
    fn get_default() -> Self;
}
/// A Struct holding all Heightmaps
#[derive(PartialEq, Eq, Debug)]
pub struct Heightmaps {
    /// A Heightmap of the heigest motion blocking block in the section
    pub motion_blocking: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest motion blocking block in the section, excluding leaves
    pub motion_blocking_no_leaves: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest ocean floor block in the section
    pub ocean_floor: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest ocean floor block in the section (use case of this is currently
    /// unknown to me)
    pub ocean_floor_wg: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest world surface block in the section
    pub world_surface: Option<HeightmapWrapper>,
    /// A Heightmap of the heigest world surface block in the section (use case of this is currently
    /// unknown to me)
    pub world_surface_wg: Option<HeightmapWrapper>,
}
impl FromNbtValue for Heightmaps {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        let motion_blocking = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let motion_blocking_no_leaves = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let ocean_floor = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let ocean_floor_wg = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let world_surface = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        let world_surface_wg = unwrap_to_empty_if_exists!(data, "MOTION_BLOCKING_NO_LEAVES", i64_array).map(|d|d.try_into().unwrap());
        Ok(Self {
            motion_blocking,
            motion_blocking_no_leaves,
            ocean_floor,
            ocean_floor_wg,
            world_surface,
            world_surface_wg,
        })
    }
}
impl Default for Heightmaps {
    fn default() -> Self {
        Self {
            motion_blocking: Default::default(),
            motion_blocking_no_leaves: Default::default(),
            ocean_floor: Default::default(),
            ocean_floor_wg: Default::default(),
            world_surface: Default::default(),
            world_surface_wg: Default::default(),
        }
    }
}
/// wrapper for heightmaps to be able to implement default and similar things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightmapWrapper {
    data: Heightmap
}
impl TryFrom<Vec<u64>> for HeightmapWrapper {
    type Error = Vec<u64>;

    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        Ok(HeightmapWrapper { data: Heightmap::try_from(value)? })
    }
}
impl TryFrom<Vec<i64>> for HeightmapWrapper {
    type Error = Vec<u64>;

    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        Ok(HeightmapWrapper { data: Heightmap::try_from(value.into_iter().map(|v| v as u64).collect::<Vec<_>>())? })
    }
}
impl Deref for HeightmapWrapper {
    type Target = Heightmap;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for HeightmapWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl Default for HeightmapWrapper {
    fn default() -> Self {
        // Provide a default implementation for T here
        // This will depend on what a reasonable default for your type T is
        // For the sake of example, let's assume you have a function to create a default instance
        HeightmapWrapper{data: Heightmap::get_default()}
    }
}
impl AsNbtValue for Heightmaps {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        todo!()
    }
}
