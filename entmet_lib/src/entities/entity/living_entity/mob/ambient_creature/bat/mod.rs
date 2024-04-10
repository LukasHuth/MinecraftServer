use super::AmbientCreature;

pub struct Bat {
    ambient_crature: AmbientCreature,
    is_hanging: bool,
}
impl Default for Bat {
    fn default() -> Self {
        Self {
            ambient_crature: AmbientCreature::default(),
            is_hanging: false,
        }
    }
}
