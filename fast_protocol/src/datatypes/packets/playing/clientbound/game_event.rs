use datatypes::Float;

/// Packet to trigger an game event
///
/// # Note
///
/// All values are sent as a `Float`
#[allow(missing_docs)]
pub enum GameEvent {
    NoRespawnBlockAvailable,
    BeginRaining,
    EndRaining,
    ChangeGamemode {
        gamemode: game_event_extra::Gamemode,
    },
    WinGame {
        roll_credits: bool,
    },
    DemoEvent {
        action: game_event_extra::DemoEventValues,
    },
    ArrowHitPlayer,
    RainLevelChange {
        value: Float,
    },
    ThunderLevelChange {
        value: Float,
    },
    PlayPufferfishStingSound,
    PlayElderGuardianMobAppearance,
    EnableRespawnScreen {
        immediately_respawn: bool,
    },
    LimitedCrafting {
        is_limited: bool,
    },
    StartWaitingForLevvelChunks,
}
/// A module containing all all extra structs for the `GameEvent`
pub mod game_event_extra {
    /// A game mode
    #[allow(missing_docs)]
    pub enum Gamemode {
        Survival,
        Creative,
        Adventure,
        Spectator,
    }
    #[repr(u8)]
    #[allow(missing_docs)]
    pub enum DemoEventValues {
        ShowWelcomeDemoScreen = 0,
        TellMovementControlls = 101,
        TellJumpControlls = 102,
        TellInventoryControl = 103,
        TellDemoIsOver = 104,
    }
}
