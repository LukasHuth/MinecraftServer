use std::ops::{Deref, DerefMut};

use super::Entity;

/// An enum of all the minecraft painting variants
///
/// # Example
/// ```rust
/// use entmet_lib::entities::entity::PaintingVariant;
/// let var1 = PaintingVariant::Alban;
/// assert_eq!(var1 as u8, 1);
/// ```
#[derive(Clone, Copy, Default)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum PaintingVariant {
    Alban = 1,
    Aztec = 2,
    Aztec2 = 3,
    Bomb = 4,
    BurningSkull = 5,
    Bust = 6,
    Courbet = 7,
    Creebet = 8,
    DonkeyKong = 9,
    Earth = 10,
    Fighters = 11,
    Fire = 12,
    Graham = 13,
    #[default] Kebab = 14,
    Match = 15,
    Pigscene = 16,
    Plant = 17,
    Pointer = 18,
    Pool = 19,
    Sea = 20,
    Skeleton = 21,
    SkullAndRoses = 22,
    Stage = 23,
    Sunset = 24,
    Void = 25,
    Wanderer = 26,
    Wasteland = 27,
    Water = 28,
    Wind = 29,
    Wither = 30,
}

/// An instance of a painting
#[derive(Default)]
pub struct Painting {
    entity: Entity,
    /// The variant of the painting
    pub var: PaintingVariant,
}
impl Deref for Painting {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl DerefMut for Painting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
