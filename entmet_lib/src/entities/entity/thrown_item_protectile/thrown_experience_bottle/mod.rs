use super::ThrownItemProtectile;


pub struct ThrownExperienceBottle {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Default for ThrownExperienceBottle {
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
