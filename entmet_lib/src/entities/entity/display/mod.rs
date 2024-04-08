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
    interpolation_delay: i32,
    transformation_interpolation_duration: i32,
    position_rotation_interpolation_duration: i32,
    translation: (f32, f32, f32),
    scale: (f32, f32, f32),
    rotation_left: (f32, f32, f32, f32),
    rotation_right: (f32, f32, f32, f32),
    billpoard_constraints: BillboardConstraints,
    brightness_override: i32,
    view_range: f32,
    shadow_radius: f32,
    shadow_strength: f32,
    width: f32,
    height: f32,
    glow_color_override: i32,
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
