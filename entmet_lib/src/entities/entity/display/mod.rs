use std::ops::{Deref, DerefMut};

use super::Entity;
pub mod block_display;
pub mod item_display;
pub mod text_display;

#[repr(u8)]
pub enum BillboardConstraints {
    Fixed = 0,
    Vertical = 1,
    Horizontal = 2,
    Center = 3,
}
pub struct Display {
    entity: Entity,
    pub interpolation_delay: i32,
    pub transformation_interpolation_duration: i32,
    pub position_rotation_interpolation_duration: i32,
    pub translation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub rotation_left: (f32, f32, f32, f32),
    pub rotation_right: (f32, f32, f32, f32),
    pub billpoard_constraints: BillboardConstraints,
    pub brightness_override: i32,
    pub view_range: f32,
    pub shadow_radius: f32,
    pub shadow_strength: f32,
    pub width: f32,
    pub height: f32,
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
            billpoard_constraints: BillboardConstraints::Fixed,
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
