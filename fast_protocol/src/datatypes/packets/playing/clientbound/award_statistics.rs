use datatypes::VarInt;

/// This will award the player an statistic
///
/// # Note
/// This will only be send, after the client send out a client command
///
/// # Note
/// The last the options are stored as a `VarInt`, that are stored inside of a `Array`
#[allow(missing_docs)]
pub struct AwardStatistics {
    pub count: VarInt,
    pub category_id: award_statistics_enums::Category,
    pub statistic_id: award_statistics_enums::Statistic,
    pub value: VarInt,
}
/// A module for all enums used in `AwardStatistics`
pub mod award_statistics_enums {
    /// The category of it
    #[allow(missing_docs)]
    #[repr(u8)]
    pub enum Category {
        Mined = 0,
        Crafted = 1,
        Used = 2,
        Broken = 3,
        PicketUp = 4,
        Dropped = 5,
        Killed = 6,
        KilledBy = 7,
        Custom = 8,
    }
    /// The statistic
    #[allow(missing_docs)]
    #[repr(u8)]
    pub enum Statistic {
        LeaveGame = 0,
        /// The played time in whole minutes ig?
        PlayOneMinute = 1,
        TimeSinceDeath = 2,
        TimeSinceRest = 3,
        SneakTime = 4,
        /// The walked distance in whole cms
        WalkOneCm = 5,
        CrouchOneCm = 6,
        SprintOneCm = 7,
        WalkOnWaterOneCm = 8,
        FallOneCm = 9,
        ClimbOneCm = 10,
        FlyOneCm = 11,
        WalkUnderwaterOneCm = 12,
        MinecartOneCm = 13,
        BoatOneCm = 14,
        PigOneCm = 15,
        HorseOneCm = 16,
        /// levitating distance
        AviateOneCm = 17,
        SwimOneCm = 18,
        StriderOneCm = 19,
        Jump = 20,
        Drop = 21,
        DamageDealt = 22,
        DamageDealtAbsorbed = 23,
        DamageDealtResisted = 24,
        DamageTaken = 25,
        DamageBlockedByShield = 26,
        DamageAbsorbed = 27,
        DamageResisted = 28,
        Deaths = 29,
        MobKills = 30,
        AnimalsBred = 31,
        PlayerKills = 32,
        FishCaught = 33,
        TalkedToVillager = 34,
        TradedWithVillager = 35,
        EatCakeSlice = 36,
        FillCauldron = 37,
        UseCauldron = 38,
        CleanArmor = 39,
        CleanBanner = 40,
        CleanShulkerBox = 41,
        InteractWithBrewingstand = 42,
        InteractWithBeacon = 43,
        InspectDropper = 44,
        InspectHopper = 45,
        InspectDispenser = 46,
        PlayNoteblock = 47,
        TuneNoteblock = 48,
        PotFlower = 49,
        TriggerTrappedChest = 50,
        OpenEnderchest = 51,
        EnchantItem = 52,
        PlayRecord = 53,
        InteractWithFurnace = 54,
        InteractWithCraftingtable = 55,
        OpenChest = 56,
        SleepInBed = 57,
        OpenShulkerBox = 58,
        OpenBarrel = 59,
        InteractWithBlastfurnace = 60,
        InteractWithSmoker = 61,
        InteractWithLectern = 62,
        InteractWithCampfire = 63,
        InteractWithCartographyTable = 64,
        InteractWithLoom = 65,
        InteractWithStonecutter = 66,
        BellRing = 67,
        RaidTriggered = 68,
        RaidWin = 69,
        InteractWithAnvil = 70,
        InteractWithGrindstone = 71,
        TargetHit = 72,
        InteractWithSmithingtable = 73,
    }
}
