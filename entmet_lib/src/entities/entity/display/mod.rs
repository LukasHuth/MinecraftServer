use std::ops::{Deref, DerefMut};

use super::Entity;
mod block_display;
pub use block_display::*;
mod item_display;
pub use item_display::*;
mod text_display;
pub use text_display::*;

/// An enum of the billboard constraints
#[repr(u8)]
#[derive(PartialEq)]
pub enum BillboardConstraints {
    /// Being fixed
    Fixed = 0,
    /// Vertically aligned
    Vertical = 1,
    /// Horizontally aligned
    Horizontal = 2,
    /// Centered aligned
    Center = 3,
}
/// An instance of a Display
#[derive(PartialEq)]
pub struct Display {
    entity: Entity,
    /// The interpolation delay
    pub interpolation_delay: i32,
    /// The transformation interpolation duration
    pub transformation_interpolation_duration: i32,
    /// the position rotation interpolation duration
    pub position_rotation_interpolation_duration: i32,
    /// The translation vector of the display
    pub translation: (f32, f32, f32),
    /// The scale of the display
    pub scale: (f32, f32, f32),
    /// The left rotation
    pub rotation_left: (f32, f32, f32, f32),
    /// The right rotation
    pub rotation_right: (f32, f32, f32, f32),
    /// The billboard constraints
    pub billboard_constraints: BillboardConstraints,
    /// The brightness override
    pub brightness_override: i32,
    /// The view range
    pub view_range: f32,
    /// The shadow radius
    pub shadow_radius: f32,
    /// The shadow strength
    pub shadow_strength: f32,
    /// The width of the display
    pub width: f32,
    /// The height of the display
    pub height: f32,
    /// Glow color override
    pub glow_color_override: i32,
}
impl Deref for Display {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
impl Default for Display {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            interpolation_delay: 0,
            transformation_interpolation_duration: 0,
            position_rotation_interpolation_duration: 0,
            translation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            rotation_left: (0.0, 0.0, 0.0, 1.0),
            rotation_right: (0.0, 0.0, 0.0, 1.0),
            billboard_constraints: BillboardConstraints::Fixed,
            brightness_override: -1,
            view_range: 1.0,
            shadow_radius: 0.0,
            shadow_strength: 1.0,
            width: 0.0,
            height: 0.0,
            glow_color_override: -1,
        }
    }
}
