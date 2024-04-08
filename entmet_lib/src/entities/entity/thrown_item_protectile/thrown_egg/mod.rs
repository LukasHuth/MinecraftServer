use super::ThrownItemProtectile;

pub struct ThrownEgg {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Default for ThrownEgg {
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
