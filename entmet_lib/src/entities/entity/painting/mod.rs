use std::ops::{Deref, DerefMut};

use super::Entity;

#[derive(Clone, Copy)]
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
    Kebab = 14,
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
pub struct Painting {
    entity: Entity,
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
impl Default for Painting {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            var: PaintingVariant::Kebab,
        }
    }
}
