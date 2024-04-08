use super::ThrownItemProtectile;


pub struct ThrownPotion {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Default for ThrownPotion {
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
