use crate::datatypes::BlockStateIdentifier;

use super::entity::{
    AreaEffectCloud, ArmorStand, Arrow, Axolotl, Bat, Bee, Blaze, BlockDisplay, Boat, Camel, Cat, ChestBoat, Chicken, Cod, Cow, Creeper, Dolphin, Donkey, DragonFireball, Drowned, ElderGuardian,
    EndCrystal, EnderDragon, Enderman, Endermite, Evoker, EvokerFangs, EyeOfEnder, FallingBlock, Fireball, FireworkRocketEntity, FishingHook, Fox, Frog, Ghast, Giant, GlowingItemFrame, Goat,
    Guardian, Hoglin, Horse, Husk, Illusioner, Interaction, IronGolem, ItemDisplay, ItemEntity, ItemFrame, Llama, LlamaSpit, Minecart, MinecartChest, MinecartCommandBlock, MinecartFurnace,
    MinecartHopper, MinecartSpawner, MinecartTnt, Mooshroom, Mule, Ocelot, Painting, Panda, Parrot, Phantom, Pig, Piglin, PiglinBrute, Pillager, Player, Polarbear, PrimedTnt, PufferFish, Rabbit,
    Ravager, Salmon, Sheep, Shulker, Skeleton, SkeletonHorse, Slime, SmallFireball, Sniffer, SnowGolem, Snowball, SpectralArrow, Spider, Squid, Stray, Strider, Tadpole, TextDisplay, ThrownEgg,
    ThrownEnderPearl, ThrownExperienceBottle, ThrownPotion, ThrownTrident, TraderLlama, TropicalFish, Turtle, Vex, Villager, Vindicator, WanderingTrader, Warden, Witch, Wither, WitherSkeleton,
    WitherSkull, Wolf, Zoglin, Zombie, ZombieHorse, ZombieVillager, ZombifiedPiglin};

/// An enum that can hold every entity of the game
pub enum EntityEnum {
    /// Enum variant to represent a `Allay`
    Allay,
    /// Enum variant to represent a `AreaEffectCloud`
    AreaEffectCloud(AreaEffectCloud),
    /// Enum variant to represent a `Armorstand`
    Armorstand(ArmorStand),
    /// Enum variant to represent a `Arrow`
    Arrow(Arrow),
    /// Enum variant to represent a `Axolotl`
    Axolotl(Axolotl),
    /// Enum variant to represent a `Bat`
    Bat(Bat),
    /// Enum variant to represent a `Bee`
    Bee(Bee),
    /// Enum variant to represent a `Blaze`
    Blaze(Blaze),
    /// Enum variant to represent a `BlockDisplay`
    ///
    /// # Note
    /// 
    /// This data is inside a `Box` to allow the T to be a type that implements
    /// `BlockStateIdentifier`
    BlockDisplay(Box<BlockDisplay<dyn BlockStateIdentifier>>),
    /// Enum variant to represent a `Boat`
    Boat(Boat),
    /// Enum variant to represent a `Camel`
    Camel(Camel),
    /// Enum variant to represent a `Cat`
    Cat(Cat),
    /// Enum variant to represent a `CaveSpider`
    CaveSpider(Spider),
    /// Enum variant to represent a `ChestBoat`
    ChestBoat(ChestBoat),
    /// Enum variant to represent a `ChestMinecart`
    ChestMinecart(MinecartChest),
    /// Enum variant to represent a `Chicken`
    Chicken(Chicken),
    /// Enum variant to represent a `Cod`
    Cod(Cod),
    /// Enum variant to represent a `CommandBlockMinecart`
    CommandBlockMinecart(MinecartCommandBlock),
    /// Enum variant to represent a `Cow`
    Cow(Cow),
    /// Enum variant to represent a `Creeper`
    Creeper(Creeper),
    /// Enum variant to represent a `Dolphin`
    Dolphin(Dolphin),
    /// Enum variant to represent a `Donkey`
    Donkey(Donkey),
    /// Enum variant to represent a `DragonFireball`
    DragonFireball(DragonFireball),
    /// Enum variant to represent a `Drowned`
    Drowned(Drowned),
    /// Enum variant to represent a `Egg`
    Egg(ThrownEgg),
    /// Enum variant to represent a `ElderGuardian`
    ElderGuardian(ElderGuardian),
    /// Enum variant to represent a `EndCrystal`
    EndCrystal(EndCrystal),
    /// Enum variant to represent a `EnderDragon`
    EnderDragon(EnderDragon),
    /// Enum variant to represent a `EnderPearl`
    EnderPearl(ThrownEnderPearl),
    /// Enum variant to represent a `Enderman`
    Enderman(Enderman),
    /// Enum variant to represent a `Endermite`
    Endermite(Endermite),
    /// Enum variant to represent a `Evoker`
    Evoker(Evoker),
    /// Enum variant to represent a `EvokerFangs`
    EvokerFangs(EvokerFangs),
    /// Enum variant to represent a `ExperienceBottle`
    ExperienceBottle(ThrownExperienceBottle),
    /// Enum variant to represent a `ExperienceOrb`
    ExperienceOrb,
    /// Enum variant to represent a `EyeOfEnder`
    EyeOfEnder(EyeOfEnder),
    /// Enum variant to represent a `FallingBlock`
    FallingBlock(FallingBlock),
    /// Enum variant to represent a `FireworkRocket`
    FireworkRocket(FireworkRocketEntity),
    /// Enum variant to represent a `Fox`
    Fox(Fox),
    /// Enum variant to represent a `Frog`
    Frog(Frog),
    /// Enum variant to represent a `FurnaceMinecart`
    FurnaceMinecart(MinecartFurnace),
    /// Enum variant to represent a `Ghast`
    Ghast(Ghast),
    /// Enum variant to represent a `Giant`
    Giant(Giant),
    /// Enum variant to represent a `GlowItemFrame`
    GlowItemFrame(GlowingItemFrame),
    /// Enum variant to represent a `GlowSquid`
    GlowSquid(Squid),
    /// Enum variant to represent a `Goat`
    Goat(Goat),
    /// Enum variant to represent a `Guardian`
    Guardian(Guardian),
    /// Enum variant to represent a `Hoglin`
    Hoglin(Hoglin),
    /// Enum variant to represent a `HopperMinecart`
    HopperMinecart(MinecartHopper),
    /// Enum variant to represent a `Horse`
    Horse(Horse),
    /// Enum variant to represent a `Husk`
    Husk(Husk),
    /// Enum variant to represent a `Illusioner`
    Illusioner(Illusioner),
    /// Enum variant to represent a `Interaction`
    Interaction(Interaction),
    /// Enum variant to represent a `IronGolem`
    IronGolem(IronGolem),
    /// Enum variant to represent a `Item`
    Item(ItemEntity),
    /// Enum variant to represent a `ItemDisplay`
    ItemDisplay(ItemDisplay),
    /// Enum variant to represent a `ItemFrame`
    ItemFrame(ItemFrame),
    /// Enum variant to represent a `Fireball`
    Fireball(Fireball),
    /// Enum variant to represent a `LeashKnot`
    LeashKnot,
    /// Enum variant to represent a `LightningBolt`
    LightningBolt,
    /// Enum variant to represent a `Llama`
    Llama(Llama),
    /// Enum variant to represent a `LlamaSpit`
    LlamaSpit(LlamaSpit),
    /// Enum variant to represent a `MagmaCube`
    MagmaCube(Slime),
    /// Enum variant to represent a `Marker`
    Marker,
    /// Enum variant to represent a `Minecart`
    Minecart(Minecart),
    /// Enum variant to represent a `Mooshroom`
    Mooshroom(Mooshroom),
    /// Enum variant to represent a `Mule`
    Mule(Mule),
    /// Enum variant to represent a `Ocelot`
    Ocelot(Ocelot),
    /// Enum variant to represent a `Painting`
    Painting(Painting),
    /// Enum variant to represent a `Panda`
    Panda(Panda),
    /// Enum variant to represent a `Parrot`
    Parrot(Parrot),
    /// Enum variant to represent a `Phantom`
    Phantom(Phantom),
    /// Enum variant to represent a `Pig`
    Pig(Pig),
    /// Enum variant to represent a `Piglin`
    Piglin(Piglin),
    /// Enum variant to represent a `PiglinBrute`
    PiglinBrute(PiglinBrute),
    /// Enum variant to represent a `Pillager`
    Pillager(Pillager),
    /// Enum variant to represent a `PolarBear`
    PolarBear(Polarbear),
    /// Enum variant to represent a `Potion`
    Potion(ThrownPotion),
    /// Enum variant to represent a `Pufferfish`
    Pufferfish(PufferFish),
    /// Enum variant to represent a `Rabbit`
    Rabbit(Rabbit),
    /// Enum variant to represent a `Ravager`
    Ravager(Ravager),
    /// Enum variant to represent a `Salmon`
    Salmon(Salmon),
    /// Enum variant to represent a `Sheep`
    Sheep(Sheep),
    /// Enum variant to represent a `Shulker`
    Shulker(Shulker),
    /// Enum variant to represent a `ShulkerBullet`
    ShulkerBullet,
    /// Enum variant to represent a `Silverfish`
    Silverfish,
    /// Enum variant to represent a `Skeleton`
    Skeleton(Skeleton),
    /// Enum variant to represent a `SkeletonHorse`
    SkeletonHorse(SkeletonHorse),
    /// Enum variant to represent a `Slime`
    Slime(Slime),
    /// Enum variant to represent a `SmallFireball`
    SmallFireball(SmallFireball),
    /// Enum variant to represent a `Sniffer`
    Sniffer(Sniffer),
    /// Enum variant to represent a `SnowGolem`
    SnowGolem(SnowGolem),
    /// Enum variant to represent a `Snowball`
    Snowball(Snowball),
    /// Enum variant to represent a `SpawnerMinecart`
    SpawnerMinecart(MinecartSpawner),
    /// Enum variant to represent a `SpectralArrow`
    SpectralArrow(SpectralArrow),
    /// Enum variant to represent a `Spider`
    Spider(Spider),
    /// Enum variant to represent a `Squid`
    Squid(Squid),
    /// Enum variant to represent a `Stray`
    Stray(Stray),
    /// Enum variant to represent a `Strider`
    Strider(Strider),
    /// Enum variant to represent a `Tadpole`
    Tadpole(Tadpole),
    /// Enum variant to represent a `TextDisplay`
    TextDisplay(TextDisplay),
    /// Enum variant to represent a `Tnt`
    Tnt(PrimedTnt),
    /// Enum variant to represent a `TntMinecart`
    TntMinecart(MinecartTnt),
    /// Enum variant to represent a `TraderLlama`
    TraderLlama(TraderLlama),
    /// Enum variant to represent a `Trident`
    Trident(ThrownTrident),
    /// Enum variant to represent a `TropicalFish`
    TropicalFish(TropicalFish),
    /// Enum variant to represent a `Turtle`
    Turtle(Turtle),
    /// Enum variant to represent a `Vex`
    Vex(Vex),
    /// Enum variant to represent a `Villager`
    Villager(Villager),
    /// Enum variant to represent a `Vindicator`
    Vindicator(Vindicator),
    /// Enum variant to represent a `WanderingTrader`
    WanderingTrader(WanderingTrader),
    /// Enum variant to represent a `Warden`
    Warden(Warden),
    /// Enum variant to represent a `Witch`
    Witch(Witch),
    /// Enum variant to represent a `Wither`
    Wither(Wither),
    /// Enum variant to represent a `WitherSkeleton`
    WitherSkeleton(WitherSkeleton),
    /// Enum variant to represent a `WitherSkull`
    WitherSkull(WitherSkull),
    /// Enum variant to represent a `Wolf`
    Wolf(Wolf),
    /// Enum variant to represent a `Zoglin`
    Zoglin(Zoglin),
    /// Enum variant to represent a `Zombie`
    Zombie(Zombie),
    /// Enum variant to represent a `ZombieHorse`
    ZombieHorse(ZombieHorse),
    /// Enum variant to represent a `ZombieVillager`
    ZombieVillager(ZombieVillager),
    /// Enum variant to represent a `ZombiePiglin`
    ZombiePiglin(ZombifiedPiglin),
    /// Enum variant to represent a `Player`
    Player(Player),
    /// Enum variant to represent a `FishingBobber`
    FishingBobber(FishingHook),
}
