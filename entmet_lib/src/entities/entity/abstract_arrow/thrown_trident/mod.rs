use super::AbstractArrow;

pub struct ThrownTrident {
    abstract_arrow: AbstractArrow,
    loyality_level: i32,
    has_enchantment_glint: bool,
}
impl Default for ThrownTrident {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            loyality_level: 0,
            has_enchantment_glint: false,
        }
    }
}
