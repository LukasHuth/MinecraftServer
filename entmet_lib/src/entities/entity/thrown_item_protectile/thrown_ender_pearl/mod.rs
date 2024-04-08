use super::ThrownItemProtectile;


pub struct ThrownEnderPearl {
    thrown_item_protectile: ThrownItemProtectile,
}
impl Default for ThrownEnderPearl{
    fn default() -> Self {
        Self {
            thrown_item_protectile: ThrownItemProtectile::default(),
        }
    }
}
