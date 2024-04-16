use super::entity::abstract_arrow::spectral_arrow::SpectralArrow;
use super::entity::abstract_arrow::thrown_trident::ThrownTrident;
use super::entity::abstract_vehicle::abstract_minecart::abstract_minecart_container::minecart_hopper::MinecartHopper;
use super::entity::abstract_vehicle::abstract_minecart::minecart::Minecart;
use super::entity::abstract_vehicle::abstract_minecart::minecart_furnace::MinecartFurnace;
use super::entity::abstract_vehicle::abstract_minecart::minecart_spawner::MinecartSpawner;
use super::entity::abstract_vehicle::abstract_minecart::minecart_tnt::MinecartTnt;
use super::entity::display::item_display::ItemDisplay;
use super::entity::display::text_display::TextDisplay;
use super::entity::dragon_fireball::DragonFireball;
use super::entity::end_crystal::EndCrystal;
use super::entity::evoker_fangs::EvokerFangs;
use super::entity::eye_of_ender::EyeOfEnder;
use super::entity::falling_block::FallingBlock;
use super::entity::fireball::Fireball;
use super::entity::firework_rocket_entity::FireworkRocketEntity;
use super::entity::fishing_hook::FishingHook;
use super::entity::interaction::Interaction;
use super::entity::item_entity::ItemEntity;
use super::entity::item_frame::glowing_item_frame::GlowingItemFrame;
use super::entity::item_frame::ItemFrame;
use super::entity::living_entity::armor_stand::ArmorStand;
use super::entity::living_entity::mob::ambient_creature::bat::Bat;
use super::entity::living_entity::mob::ender_dragon::EnderDragon;
use super::entity::living_entity::mob::flying::ghast::Ghast;
use super::entity::living_entity::mob::flying::phantom::Phantom;
use super::entity::living_entity::mob::pathfinder_mob::abstract_golem::iron_golem::IronGolem;
use super::entity::living_entity::mob::pathfinder_mob::abstract_golem::shulker::Shulker;
use super::entity::living_entity::mob::pathfinder_mob::abstract_golem::snow_golem::SnowGolem;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::abstract_villager::villager::Villager;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::abstract_villager::wandering_trader::WanderingTrader;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::chested_horse::donkey::Donkey;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::chested_horse::llama::trader_llama::TraderLlama;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::chested_horse::llama::Llama;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::chested_horse::mule::Mule;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::horse::Horse;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::skeleton_horse::SkeletonHorse;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::abstract_horse::zombie_horse::ZombieHorse;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::cow::mooshroom::Mooshroom;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::cow::Cow;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::fox::Fox;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::frog::Frog;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::goat::Goat;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::hoglin::Hoglin;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::ocelot::Ocelot;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::panda::Panda;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::pig::Pig;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::polar_bear::Polarbear;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::rabbit::Rabbit;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::sheep::Sheep;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::sniffer::Sniffer;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::strider::Strider;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::tameable_animal::parrot::Parrot;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::tameable_animal::wolf::Wolf;
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::turtle::Turtle;
use super::entity::living_entity::mob::pathfinder_mob::monster::abstract_skeleton::skeleton::Skeleton;
use super::entity::living_entity::mob::pathfinder_mob::monster::abstract_skeleton::stray::Stray;
use super::entity::living_entity::mob::pathfinder_mob::monster::abstract_skeleton::wither_skeleton::WitherSkeleton;
use super::entity::living_entity::mob::pathfinder_mob::monster::base_piglin::piglin::Piglin;
use super::entity::living_entity::mob::pathfinder_mob::monster::base_piglin::piglin_brute::PiglinBrute;
use super::entity::living_entity::mob::pathfinder_mob::monster::creeper::Creeper;
use super::entity::living_entity::mob::pathfinder_mob::monster::enderman::Enderman;
use super::entity::living_entity::mob::pathfinder_mob::monster::endermite::Endermite;
use super::entity::living_entity::mob::pathfinder_mob::monster::giant::Giant;
use super::entity::living_entity::mob::pathfinder_mob::monster::guardian::elder_guardian::ElderGuardian;
use super::entity::living_entity::mob::pathfinder_mob::monster::guardian::Guardian;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::abstract_illager::pillager::Pillager;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::abstract_illager::spellcaster_illager::evoker::Evoker;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::abstract_illager::spellcaster_illager::illusioner::Illusioner;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::abstract_illager::vindicator::Vindicator;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::ravager::Ravager;
use super::entity::living_entity::mob::pathfinder_mob::monster::raider::witch::Witch;
use super::entity::living_entity::mob::pathfinder_mob::monster::vex::Vex;
use super::entity::living_entity::mob::pathfinder_mob::monster::warden::Warden;
use super::entity::living_entity::mob::pathfinder_mob::monster::wither::Wither;
use super::entity::living_entity::mob::pathfinder_mob::monster::zoglin::Zoglin;
use super::entity::living_entity::mob::pathfinder_mob::monster::zombie::drowned::Drowned;
use super::entity::living_entity::mob::pathfinder_mob::monster::zombie::husk::Husk;
use super::entity::living_entity::mob::pathfinder_mob::monster::zombie::zombie_villager::ZombieVillager;
use super::entity::living_entity::mob::pathfinder_mob::monster::zombie::zombified_piglin::ZombifiedPiglin;
use super::entity::living_entity::mob::pathfinder_mob::monster::zombie::Zombie;
use super::entity::living_entity::mob::pathfinder_mob::monster::{blaze::Blaze, spider::Spider};
use super::entity::living_entity::mob::pathfinder_mob::ageable_mob::animal::{abstract_horse::camel::Camel, axolotl::Axolotl, bee::Bee, chicken::Chicken, tameable_animal::cat::Cat};
use super::entity::display::block_display::BlockDisplay;
use super::entity::area_effect_cloud::AreaEffectCloud;
use super::entity::abstract_vehicle::boat::{chest_boat::ChestBoat, Boat};
use super::entity::abstract_vehicle::abstract_minecart::{abstract_minecart_container::minecart_chest::MinecartChest, minecart_command_block::MinecartCommandBlock};
use super::entity::abstract_arrow::arrow::Arrow;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::abstract_fish::cod::Cod;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::abstract_fish::puffer_fish::PufferFish;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::abstract_fish::salmon::Salmon;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::abstract_fish::tadpole::Tadpole;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::abstract_fish::tropical_fish::TropicalFish;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::dolphin::Dolphin;
use super::entity::living_entity::mob::pathfinder_mob::water_animal::squid::Squid;
use super::entity::living_entity::mob::slime::Slime;
use super::entity::living_entity::player::Player;
use super::entity::llama_spit::LlamaSpit;
use super::entity::painting::Painting;
use super::entity::primed_tnt::PrimedTnt;
use super::entity::small_fireball::SmallFireball;
use super::entity::thrown_item_protectile::snowball::Snowball;
use super::entity::thrown_item_protectile::thrown_egg::ThrownEgg;
use super::entity::thrown_item_protectile::thrown_ender_pearl::ThrownEnderPearl;
use super::entity::thrown_item_protectile::thrown_experience_bottle::ThrownExperienceBottle;
use super::entity::thrown_item_protectile::thrown_potion::ThrownPotion;
use super::entity::wither_skull::WitherSkull;
use super::entity::Entity;

pub enum EntityEnum {
    Allay,
    AreaEffectCloud(AreaEffectCloud),
    Armorstand(ArmorStand),
    Arrow(Arrow),
    Axolotl(Axolotl),
    Bat(Bat),
    Bee(Bee),
    Blaze(Blaze),
    BlockDisplay(BlockDisplay),
    Boat(Boat),
    Camel(Camel),
    Cat(Cat),
    CaveSpider(Spider),
    ChestBoat(ChestBoat),
    ChestMinecart(MinecartChest),
    Chicken(Chicken),
    Cod(Cod),
    CommandBlockMinecart(MinecartCommandBlock),
    Cow(Cow),
    Creeper(Creeper),
    Dolphin(Dolphin),
    Donkey(Donkey),
    DragonFireball(DragonFireball),
    Drowned(Drowned),
    Egg(ThrownEgg),
    ElderGuardian(ElderGuardian),
    EndCrystal(EndCrystal),
    EnderDragon(EnderDragon),
    EnderPearl(ThrownEnderPearl),
    Enderman(Enderman),
    Endermite(Endermite),
    Evoker(Evoker),
    EvokerFangs(EvokerFangs),
    ExperienceBottle(ThrownExperienceBottle),
    ExperienceOrb,
    EyeOfEnder(EyeOfEnder),
    FallingBlock(FallingBlock),
    FireworkRocket(FireworkRocketEntity),
    Fox(Fox),
    Frog(Frog),
    FurnaceMinecart(MinecartFurnace),
    Ghast(Ghast),
    Giant(Giant),
    GlowItemFrame(GlowingItemFrame),
    GlowSquid(Squid),
    Goat(Goat),
    Guardian(Guardian),
    Hoglin(Hoglin),
    HopperMinecart(MinecartHopper),
    Horse(Horse),
    Husk(Husk),
    Illusioner(Illusioner),
    Interaction(Interaction),
    IronGolem(IronGolem),
    Item(ItemEntity),
    ItemDisplay(ItemDisplay),
    ItemFrame(ItemFrame),
    Fireball(Fireball),
    LeashKnot,
    LightningBolt,
    Llama(Llama),
    LlamaSpit(LlamaSpit),
    MagmaCube(Slime),
    Marker,
    Minecart(Minecart),
    Mooshroom(Mooshroom),
    Mule(Mule),
    Ocelot(Ocelot),
    Painting(Painting),
    Panda(Panda),
    Parrot(Parrot),
    Phantom(Phantom),
    Pig(Pig),
    Piglin(Piglin),
    PiglinBrute(PiglinBrute),
    Pillager(Pillager),
    PolarBear(Polarbear),
    Potion(ThrownPotion),
    Pufferfish(PufferFish),
    Rabbit(Rabbit),
    Ravager(Ravager),
    Salmon(Salmon),
    Sheep(Sheep),
    Shulker(Shulker),
    ShulkerBullet,
    Silverfish,
    Skeleton(Skeleton),
    SkeletonHorse(SkeletonHorse),
    Slime(Slime),
    SmallFireball(SmallFireball),
    Sniffer(Sniffer),
    SnowGolem(SnowGolem),
    Snowball(Snowball),
    SpawnerMinecart(MinecartSpawner),
    SpectralArrow(SpectralArrow),
    Spider(Spider),
    Squid(Squid),
    Stray(Stray),
    Strider(Strider),
    Tadpole(Tadpole),
    TextDisplay(TextDisplay),
    Tnt(PrimedTnt),
    TntMinecart(MinecartTnt),
    TraderLlama(TraderLlama),
    Trident(ThrownTrident),
    TropicalFish(TropicalFish),
    Turtle(Turtle),
    Vex(Vex),
    Villager(Villager),
    Vindicator(Vindicator),
    WanderingTrader(WanderingTrader),
    Warden(Warden),
    Witch(Witch),
    Wither(Wither),
    WitherSkeleton(WitherSkeleton),
    WitherSkull(WitherSkull),
    Wolf(Wolf),
    Zoglin(Zoglin),
    Zombie(Zombie),
    ZombieHorse(ZombieHorse),
    ZombieVillager(ZombieVillager),
    ZombiePiglin(ZombifiedPiglin),
    Player(Player),
    FishingBobber(FishingHook),
}
