use std::ops::{Deref, DerefMut};

use super::AbstractVehicle;

pub mod minecart;
pub mod abstract_minecart_container;
pub mod minecart_furnace;
pub mod minecart_tnt;
pub mod minecart_spawner;
pub mod minecart_command_block;

pub struct AbstractMinecart {
    abstract_vehicle: AbstractVehicle,
    pub custom_block_id: i32,
    pub custom_block_y: i32,
    pub show_custom_block: bool,
}
impl Default for AbstractMinecart {
    fn default() -> Self {
        Self {
            abstract_vehicle: AbstractVehicle::default(),
            custom_block_id: 0,
            custom_block_y: 6,
            show_custom_block: false,
        }
    }
}
impl Deref for AbstractMinecart {
    type Target = AbstractVehicle;

    fn deref(&self) -> &Self::Target {
        &self.abstract_vehicle
    }
}
impl DerefMut for AbstractMinecart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_vehicle
    }
}
