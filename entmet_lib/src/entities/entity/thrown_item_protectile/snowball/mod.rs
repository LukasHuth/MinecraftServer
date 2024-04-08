use super::ThrownItemProtectile;

pub struct Snowball {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Default for Snowball{
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
