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
    pub category_id: AwardStatisticsCategory,
    pub statistic_id: VarInt,
    pub value: VarInt,
}
/// The category of it
#[allow(missing_docs)]
#[repr(u8)]
pub enum AwardStatisticsCategory {
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
