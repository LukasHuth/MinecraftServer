/// An enum of all villager types.
#[derive(Default)]
#[repr(u8)]
pub enum VillagerType {
    /// Villager from a desert village.
    Desert = 0,
    /// Villager from a jungle village.
    Jungle = 1,
    /// Villager from a plains village, this is the default
    #[default]
    Plains = 2,
    /// Villager from a savanna village.
    Savanna = 3,
    /// Villager from a snow village.
    Snow = 4,
    /// Villager from a swamp village.
    Swamp = 5,
    /// Villager from a taiga village.
    Taiga = 6,
}
/// An enum of all villager profession
#[repr(u8)]
pub enum VillagerProfession {
    /// Variant if no profession is assigned, but assignable
    None = 0,
    /// Villager producing Armor etc.
    Armorer = 1,
    /// Villager that sells meat
    Butcher = 2,
    /// Villager that sells maps etc.
    Cartographer = 3,
    /// Villager that sells potion stuff like glow stone
    Cleric = 4,
    /// Villager that sells plants and food made of them
    Farmer = 5,
    /// villager that sells Fish and stuff associated with them
    Fisherman = 6,
    /// villager that sells wood and things like that
    Fletcher = 7,
    /// villager that sells leather stuff etc.
    Leatherworker = 8,
    /// villager that sells enchanted books etc.
    Librarian = 9,
    /// villager that sells stone related stuff
    Mason = 10,
    /// Villager variant that can't learn a job
    Nitwin = 11,
    /// villager that sells wool etc.
    Shepherd = 12,
    /// villager that sells tools
    Toolsmith = 13,
    /// villager that sells weapons
    Weaponsmith = 14,
}
