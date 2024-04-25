/// An enum of all Particle types
pub enum ParticleType {
    /// A particle of type `AmbientEntityEffect`
    AmbientEntityEffect,
    /// A particle of type `AngryVillager`
    AngryVillager,
    /// A particle of type `Block`
    Block,
    /// A particle of type `BlockMarker`
    BlockMarker,
    /// A particle of type `Bubble`
    Bubble,
    /// A particle of type `Cloud`
    Cloud,
    /// A particle of type `Crit`
    Crit,
    /// A particle of type `DamageIndicator`
    DamageIndicator,
    /// A particle of type `DragonBreath`
    DragonBreath,
    /// A particle of type `DrippingLava`
    DrippingLava,
    /// A particle of type `FallingLava`
    FallingLava,
    /// A particle of type `LandingLava`
    LandingLava,
    /// A particle of type `DrippingWater`
    DrippingWater,
    /// A particle of type `FallingWater`
    FallingWater,
    /// A particle of type `Dust`
    Dust,
    /// A particle of type `DustColorTransition`
    DustColorTransition,
    /// A particle of type `Effect`
    Effect,
    /// A particle of type `ElderGuardian`
    ElderGuardian,
    /// A particle of type `EnchantedHit`
    EnchantedHit,
    /// A particle of type `Enchant`
    Enchant,
    /// A particle of type `EndRod`
    EndRod,
    /// A particle of type `EntityEffect`
    EntityEffect,
    /// A particle of type `ExplosionEmitter`
    ExplosionEmitter,
    /// A particle of type `Explosion`
    Explosion,
    /// A particle of type `Gust`
    Gust,
    /// A particle of type `GustEmitter`
    GustEmitter,
    /// A particle of type `SonicBoom`
    SonicBoom,
    /// A particle of type `FallingDust`
    FallingDust,
    /// A particle of type `Firework`
    Firework,
    /// A particle of type `Fishing`
    Fishing,
    /// A particle of type `Flame`
    Flame,
    /// A particle of type `CherryLeaves`
    CherryLeaves,
    /// A particle of type `SculkSoul`
    SculkSoul,
    /// A particle of type `SculkCharge`
    SculkCharge,
    /// A particle of type `SculkChargePop`
    SculkChargePop,
    /// A particle of type `SoulFireFlame`
    SoulFireFlame,
    /// A particle of type `Soul`
    Soul,
    /// A particle of type `Flash`
    Flash,
    /// A particle of type `HappyVillager`
    HappyVillager,
    /// A particle of type `Composter`
    Composter,
    /// A particle of type `Heart`
    Heart,
    /// A particle of type `InstantEffect`
    InstantEffect,
    /// A particle of type `Item`
    Item,
    /// A particle of type `Vibration`
    Vibration,
    /// A particle of type `ItemSlime`
    ItemSlime,
    /// A particle of type `ItemSnowball`
    ItemSnowball,
    /// A particle of type `LargeSmoke`
    LargeSmoke,
    /// A particle of type `Lava`
    Lava,
    /// A particle of type `Mycellium`
    Mycellium,
    /// A particle of type `Note`
    Note,
    /// A particle of type `Poof`
    Poof,
    /// A particle of type `Portal`
    Portal,
    /// A particle of type `Rain`
    Rain,
    /// A particle of type `Smoke`
    Smoke,
    /// A particle of type `WhiteSmoke`
    WhiteSmoke,
    /// A particle of type `Sneeze`
    Sneeze,
    /// A particle of type `Spit`
    Spit,
    /// A particle of type `SquidInk`
    SquidInk,
    /// A particle of type `SweepAttack`
    SweepAttack,
    /// A particle of type `TotemOfUndying`
    TotemOfUndying,
    /// A particle of type `Underwater`
    Underwater,
    /// A particle of type `Splash`
    Splash,
    /// A particle of type `Witch`
    Witch,
    /// A particle of type `BubblePop`
    BubblePop,
    /// A particle of type `CurrentDown`
    CurrentDown,
    /// A particle of type `BubbleColumnUp`
    BubbleColumnUp,
    /// A particle of type `Nautilus`
    Nautilus,
    /// A particle of type `Dolphin`
    Dolphin,
    /// A particle of type `CampfireSignalSmoke`
    CampfireSignalSmoke,
    /// A particle of type `DrippingHoney`
    DrippingHoney,
    /// A particle of type `FallingHoney`
    FallingHoney,
    /// A particle of type `LandingHoney`
    LandingHoney,
    /// A particle of type `FallingNectar`
    FallingNectar,
    /// A particle of type `FallingSporeBlossom`
    FallingSporeBlossom,
    /// A particle of type `Ash`
    Ash,
    /// A particle of type `CrimsonSpore`
    CrimsonSpore,
    /// A particle of type `WarpedSpore`
    WarpedSpore,
    /// A particle of type `SporeBlossomAir`
    SporeBlossomAir,
    /// A particle of type `DrippingObsidianTear`
    DrippingObsidianTear,
    /// A particle of type `FallingObsidianTear`
    FallingObsidianTear,
    /// A particle of type `LandingObsidianTear`
    LandingObsidianTear,
    /// A particle of type `ReversePortal`
    ReversePortal,
    /// A particle of type `WhiteAsh`
    WhiteAsh,
    /// A particle of type `SmallFlame`
    SmallFlame,
    /// A particle of type `Snowflake`
    Snowflake,
    /// A particle of type `DrippingDripstoneLava`
    DrippingDripstoneLava,
    /// A particle of type `FallingDripstoneLava`
    FallingDripstoneLava,
    /// A particle of type `DrippingDripstoneWater`
    DrippingDripstoneWater,
    /// A particle of type `FallingDripstoneWater`
    FallingDripstoneWater,
    /// A particle of type `GlowSquidInk`
    GlowSquidInk,
    /// A particle of type `Glow`
    Glow,
    /// A particle of type `WaxOn`
    WaxOn,
    /// A particle of type `WaxOff`
    WaxOff,
    /// A particle of type `ElectricSpark`
    ElectricSpark,
    /// A particle of type `Scrape`
    Scrape,
    /// A particle of type `Shriek`
    Shriek,
    /// A particle of type `EggCrack`
    EggCrack,
    /// A particle of type `DustPlume`
    DustPlume,
    /// A particle of type `GustDust`
    GustDust,
    /// A particle of type `TrialSpawnerDetection`
    TrialSpawnerDetection,
}
