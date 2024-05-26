use nbt_lib::traits::FromNbtValue;

use nbt_lib::{convert_to_bool, unwrap_to_empty};

use super::Orientation;

/// Represents an element of a jungle temple.
///
/// Each element contains information such as its bounding box, depth, generation distance,
/// whether hidden and main chests are placed, whether traps are placed, height, horizontal position,
/// unique identifier, and orientation.
#[derive(PartialEq, Eq, Debug)]
pub struct JungleTempleElement {
    /// The bounding box of the jungle temple element.
    pub bounding_box: [i32;6],
    /// The depth of the jungle temple element.
    pub depth: i32,
    /// The generation distance of the jungle temple element.
    pub generation_distance: i32,
    /// Indicates whether a hidden chest is placed in the jungle temple element.
    pub placed_hidden_chest: bool,
    /// Indicates whether the main chest is placed in the jungle temple element.
    pub placed_main_chest: bool,
    /// Indicates whether the first trap is placed in the jungle temple element.
    pub placed_trap_1: bool,
    /// Indicates whether the second trap is placed in the jungle temple element.
    pub placed_trap_2: bool,
    /// The height of the jungle temple element.
    pub height: i32,
    /// The horizontal position of the jungle temple element.
    pub h_pos: i32,
    /// The unique identifier of the jungle temple element.
    pub id: String,
    /// The orientation of the jungle temple element.
    pub orientation: Orientation,
}
impl FromNbtValue for JungleTempleElement {
    fn from_nbt_value(value: nbt_lib::NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        Ok(Self {
            bounding_box: unwrap_to_empty!(data.get("BB"), i32_array).try_into().unwrap(),
            depth: unwrap_to_empty!(data.get("Depth"), i32),
            generation_distance: unwrap_to_empty!(data.get("DG"), i32),
            placed_hidden_chest: convert_to_bool!(unwrap_to_empty!(data.get("placedHiddenChest"), i8)),
            placed_main_chest: convert_to_bool!(unwrap_to_empty!(data.get("placedMainChest"), i8)),
            placed_trap_1: convert_to_bool!(unwrap_to_empty!(data.get("placedTrap1"), i8)),
            placed_trap_2: convert_to_bool!(unwrap_to_empty!(data.get("placedTrap2"), i8)),
            height: unwrap_to_empty!(data.get("Height"), i32),
            h_pos: unwrap_to_empty!(data.get("HPos"), i32),
            id: unwrap_to_empty!(data.get("id"), string),
            orientation: Orientation::from(unwrap_to_empty!(data.get("O"), i32))
        })
    }
}
