use std::ops::{Deref, DerefMut};

use super::AbstractVehicle;

mod minecart;
pub use minecart::*;
mod abstract_minecart_container;
pub use abstract_minecart_container::*;
mod minecart_furnace;
pub use minecart_furnace::*;
mod minecart_tnt;
pub use minecart_tnt::*;
mod minecart_spawner;
pub use minecart_spawner::*;
mod minecart_command_block;
pub use minecart_command_block::*;

/// An interface to store minecart data
pub struct AbstractMinecart {
    abstract_vehicle: AbstractVehicle,
    /// The costom block id
    pub custom_block_id: i32,
    /// The custom Y-Position of the block ( in 16ths of a block )
    pub custom_block_y: i32,
    /// Whether the custom block should be shown
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
