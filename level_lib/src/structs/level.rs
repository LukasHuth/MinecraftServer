use std::collections::HashMap;
use std::time::SystemTime;

use super::generator::Generator;

/// An enum of all possible bossbar overlay options
#[allow(missing_docs)]
pub enum Overlay {
    Progress,
    Notched6,
    Notched10,
    Notched12,
    Notched20,
}
impl Overlay {
    /// function to convert to a string
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Notched6 => "notched_6",
            Self::Notched10 => "notched_10",
            Self::Notched12 => "notched_12",
            Self::Notched20 => "notched_20",
        }
    }
}

/// An enum of the possible difficulties
#[repr(u8)]
pub enum Difficulty {
    /// Difficulty peaceful
    Peaceful = 0,
    /// Difficulty easy
    Easy = 1,
    /// Difficulty normal
    Normal = 2,
    /// Difficulty hard
    Hard = 3,
}
/// An enum of the possible difficulties
#[repr(u8)]
pub enum GameMode {
    /// Gamemode survival
    Survival = 0,
    /// Gamemode creative
    Creative = 1,
    /// Gamemode adventure
    Adventure = 2,
    /// Gamemode spectator
    Spectator = 3,
}
/// Structure to hold bossbar information
pub struct CustomBossEvent {
    /// The id of the bossbar (e.g.`custom:boss`)
    pub id: String,
    /// A list of players that may see this boss bar
    pub players: Vec<u128>,
    /// The color id of the bossbar color
    pub color: String,
    /// If the bossbar should create fog
    pub create_world_fog: bool,
    /// If the bossbar should darken the sky
    pub darken_screen: bool,
    /// The maximum health of the bossbar
    pub max: i32,
    /// The current health of the bossbar
    pub value: i32,
    /// The displayed name of the bossbar as
    pub name: datatypes::JSONTextComponent,
    /// The overlay shown over the healthbar
    pub overlay: Overlay,
    /// If the boosbar should initiate boss music
    pub play_boss_music: bool,
    /// If the bossbar should be visible to the listed players
    pub visible: bool,
}
/// Structure to hold End data
pub struct EndData {
    /// The x coordinate of the portal
    pub exit_portal_location_x: i8,
    /// The y coordinate of the portal
    pub exit_portal_location_y: i8,
    /// The z coordinate of the portal
    pub exit_portal_location_z: i8,
    /// A list of all end gateway portals that haven't been spawned
    pub future_gateways: Vec<i32>,
    /// If the dragon is currently alive
    pub dragon_killed: bool,
    /// The dragon uuid
    pub dragon_uuid: u128,
    /// If the ender dragon has ever been defeated
    pub previously_killed: bool,
}
/// Structure to hold the imporant but basic Level data
pub struct Level {
    /// If cheats are enabled
    pub allow_cheats: bool,
    /// The x position of the center of the border
    pub border_center_x: f64,
    /// The y position of the center of the border
    pub border_center_y: f64,
    /// The damage dealt by the border per block outside
    pub border_damage_per_block: f64,
    /// The size of the border
    pub border_size: f64,
    /// The safe zone of the border
    pub border_safe_zone: f64,
    /// The lerp target size
    pub boder_size_lerp_target: f64,
    /// The lerp target time
    pub border_size_lerp_time: i64,
    /// The block length of the warning
    pub border_warning_blocks: f64,
    /// The time of the warning
    pub border_warning_time: f64,
    /// The time until clear weather has ended
    pub clear_weather_time: i32,
    /// A vollection of active bossbars
    pub custom_boss_events: Vec<CustomBossEvent>,
    /// List of disabled datapacks
    pub disabled: Vec<String>,
    /// List of enabled datapacks
    pub enabled: Vec<String>,
    /// The Data version
    pub data_version: i32,
    /// The time of the day
    ///
    /// # Note
    ///
    /// This counts continiously up to determine the day
    pub day_time: i64,
    /// The difficulty
    pub difficulty: Difficulty,
    /// If the difficulty is locked
    pub difficulty_locked: bool,
    /// End data
    ///
    /// # Note
    ///
    /// This is only existent, after the end is entered for the first time
    pub end_data: Option<EndData>,
    /// A list of the used game rules
    pub game_rules: HashMap<String, String>,
    /// The world generation settings for every dimension
    pub world_gen_settings: WorldGenSettings,
    /// The default game mode of players
    pub default_game_mode: GameMode,
    /// Wether the player is allowed to respawn after the death
    pub hardcore: bool,
    /// Whether the world was initiated right or not
    pub initialized: bool,
    /// The timestamp, when this map was last loaded
    ///
    /// # Note
    ///
    /// Saved as Unix time in milliseconds
    pub last_played: SystemTime,
    /// The name of the level
    pub level_name: String,
    /// Wether structures should be generated or not
    pub map_features: bool,
    // player is not stored, because every player has its own data file
    /// Whether it is raining or not
    pub raining: bool,
    /// Time until the state of `raining` is toggled again
    pub rain_time: i32,
    /// The seed used for continueus level generation
    pub random_seed: i64,
    /// An estimated size, currently unused, so null in this implementation
    pub size_on_disk: i64,
    /// The x coordinate of the spawn
    pub spawn_x: i32,
    /// The y coordinate of the spawn
    pub spawn_y: i32,
    /// The z coordinate of the spawn
    pub spawn_z: i32,
    /// Whether it is thundering or not
    ///
    /// # Note
    ///
    /// This only applies, if `raining` is true and it is dark enough so that monsters are able to
    /// spawn below the sky
    pub thundering: bool,
    /// The time until `thundering` is toggled again
    pub thunder_time: i32,
    /// The NBT version of the level
    pub version: i32,
    /// The minecraft version, the world was saved in
    pub mc_version: Version,
    /// Id of the current wandering traider in the world
    pub wandering_trader_id: u128,
    /// The current chance of a wandering trader spawning
    pub wandering_trader_spawn_chance: f64,
    /// The amount of ticks until the next wandering trader spawn try is triggered
    pub wandering_trader_spawn_delay: i32,
    /// If the was saved in a modified version or not
    pub was_modified: bool,
}
/// A struct storing minecraft version data
pub struct Version {
    /// The version id
    pub id: i32,
    /// The version name
    pub name: String,
    /// The version series, mostly "main"
    pub series: String,
    /// whether the version is a snapshot or not
    pub snapshot: bool,
}
/// A struct holding the world gen settings
pub struct WorldGenSettings {
    /// If a bonus chest should be generated
    pub bonus_chest: bool,
    /// The seed of the world
    pub seeed: i64,
    /// Whether structures should be generated or not
    pub generate_features: bool,
    /// A list containing all dimensions
    pub dimensions: HashMap<String, Generator>,
}
/// A struct storing all the dimension data
pub struct Dimension {
    /// The id of the dimension
    pub id: String,
}
