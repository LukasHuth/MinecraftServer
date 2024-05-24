use nbt_lib::traits::FromNbtValue;

use nbt_lib::{convert_to_bool, unwrap_to_empty};

use super::Orientation;

pub struct JungleTempleElement {
    pub bounding_box: [i32;6],
    pub depth: i32,
    pub generation_distance: i32,
    pub placed_hidden_chest: bool,
    pub placed_main_chest: bool,
    pub placed_trap_1: bool,
    pub placed_trap_2: bool,
    pub height: i32,
    pub h_pos: i32,
    pub id: String,
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
