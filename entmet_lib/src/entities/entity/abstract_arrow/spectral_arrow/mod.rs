use super::AbstractArrow;

pub struct SpectralArrow {
    abstract_arrow: AbstractArrow,
}
impl Default for SpectralArrow {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
        }
    }
}
