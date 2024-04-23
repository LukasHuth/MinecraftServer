//! Module containing biome data strucs

use std::ops::{RangeBounds};

/// A ranged struct 
pub struct RangedData {
    /// The min value
    min: f32,
    /// The max value
    max: f32,
}
impl RangedData {
    /// sets the `min` value and checks, if it is inside of the bounds
    pub fn set_min(&mut self, value: f32) {
        if value >= -2.0 && self.max >= value {
            self.min = value;
        }
    }
    /// sets the `max` value and checks, if it is inside of the bounds
    pub fn set_max(&mut self, value: f32) {
        if value <= 2.0 && self.min <= value {
            self.min = value;
        }
    }
    /// gets the `min` value
    pub fn get_min(&self) -> f32 { self.min }
    /// gets the `max` value
    pub fn get_max(&self) -> f32 { self.max }
    /// creates a new instance of `RangedData`
    pub fn new(min: f32, max: f32) -> Self {
        if min <= max && min >= -2.0 && max <= 2.0 {
            Self { min, max }
        } else {
            Self { min: 0.0, max: 0.0 }
        }
    }
}
impl RangeBounds<f32> for RangedData {
    fn start_bound(&self) -> std::ops::Bound<&f32> {
        std::ops::Bound::Included(&-2.0)
    }

    fn end_bound(&self) -> std::ops::Bound<&f32> {
        std::ops::Bound::Included(&2.0)
    }
}
/// A struct storing biome data
pub struct Biome {
    /// The biome id
    pub id: String,
    /// Data storing the `temperature` as a min/max pair
    pub temperature: RangedData,
    /// Data storing the `humidity` as a min/max pair
    pub humidity: RangedData,
    /// Data storing the `continentalneess` as a min/max pair
    pub continentalneess: RangedData,
    /// Data storing the `erosion` as a min/max pair
    pub erosion: RangedData,
    /// Data storing the `weirdness` as a min/max pair
    pub weirdness: RangedData,
    /// Data storing the `depth` as a min/max pair
    pub depth: RangedData,
    /// The Offset (use case currently unknown to me)
    pub offset: f32,
}
