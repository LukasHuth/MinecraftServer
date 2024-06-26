//! A module for all serverbound packets that are beeing send while playing

mod implementations;

use datatypes::{
    BitMask, Boolean, Byte, ByteArray, Double, FixedBitSet, FixedByteArray, Float, Identifier, Int,
    Long, UnsignedByte, VarInt, UUID,
};
/// Confirms a teleportation packet
pub struct ConfirmTeleportation {
    /// The id of the `SynchronizePlayerPosition` Packet
    pub t_id: VarInt,
}
/// Query Block Entity Tag
#[deprecated]
pub struct QueryBlockEntityTag;
mod change_difficulty;
pub use change_difficulty::*;
/// Acknowledges Messages
pub struct AcknowledgeMessage {
    /// The count of accepted messages
    pub message_count: VarInt,
}
/// A command, send by the client
pub struct ChatCommand {
    /// The command
    pub command: datatypes::String,
}
mod signed_chat_command;
pub use signed_chat_command::*;
/// Chat Message
#[allow(missing_docs)]
pub struct ChatMessage {
    pub message: datatypes::String,
    pub timestamp: Long,
    pub salt: Long,
    pub has_signature: Boolean,
    pub signature: Option<FixedByteArray<256>>,
    pub message_count: VarInt,
    pub acknowledged: FixedBitSet<20>,
}
/// PlayerSession
#[deprecated]
pub struct PlayerSession;
/// Chunk batch received
///
/// More Info: <https://wiki.vg/Protocol#Chunk_Batch_Received>
pub struct ChunkBatchReceived {
    /// The chunks that the client recieved per tick
    pub chunks_per_tick: Float,
}
mod client_status;
pub use client_status::*;
mod client_information;
pub use client_information::*;
/// Command Suggestion Request
#[allow(missing_docs)]
pub struct CommandSuggestionRequest {
    pub transaction_id: VarInt,
    pub text: datatypes::String,
}
/// Acklowledge Configuration
pub struct AcknowledgeConfiguration;
/// Click Container Button
#[deprecated]
pub struct ClickContainerButton;
/// ClickContainer
#[deprecated]
pub struct ClickContainer;
/// Close a Container by the id
pub struct CloseContainer {
    /// The window id
    pub window_id: UnsignedByte,
}
/// Change Container Slot State
#[deprecated]
pub struct ChangeContainerSlotState;
/// Cookie Response
#[deprecated]
pub struct CookieResponse;
/// A Plugin Message
pub struct PluginMessage {
    /// the channel that the message is getting send to
    pub channel: Identifier,
    /// The data of the plugin message
    pub data: ByteArray,
}
/// Debug Sample Subscription
#[deprecated]
pub struct DebugSampleSubscription;
/// EditBook
#[deprecated]
pub struct EditBook;
/// Query Entity Tag
#[deprecated]
pub struct QueryEntityTag;
/// Interact
#[deprecated]
pub struct Interact;
/// Jigsaw Generate
///
/// More Information: <https://wiki.vg/Protocol#Jigsaw_Generate>
#[deprecated]
pub struct JigsawGenerate;
/// Keep Alive
pub struct KeepAlive {
    /// The id
    pub id: Long,
}
/// Lock Difficulty
pub struct LockDifficulty {
    /// Whether it shold be locked or not
    pub locket: Boolean,
}
/// Sets the player location
#[allow(missing_docs)]
pub struct SetPlayerPosition {
    pub x: Double,
    pub feet_y: Double,
    pub z: Double,
    pub on_ground: Boolean,
}
#[allow(missing_docs)]
pub struct SetPlayerPositionAndRotation {
    pub x: Double,
    pub feet_y: Double,
    pub z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub on_ground: Boolean,
}
#[allow(missing_docs)]
pub struct SetPlayerRotation {
    pub yaw: Float,
    pub pitch: Float,
    pub on_ground: Boolean,
}
/// Set Player On Ground
pub struct SetPlayerOnGround {
    /// Whether the player is on the ground or not
    pub on_ground: Boolean,
}
/// Move Vehicle
#[deprecated]
pub struct MoveVehicle;
/// PaddleBoat
#[deprecated]
pub struct PaddleBoat;
/// Pick Item
#[deprecated]
pub struct PickItem;
/// A Ping Request
pub struct PingRequest {
    /// a semi random payload
    pub payload: Long,
}
/// Place Recipe
#[deprecated]
pub struct PlaceRecipe;
/// Player Abilities
pub struct PlayerAbilities {
    /// The flags
    pub flags: BitMask<Byte, super::clientbound::player_abilities_extra::PlayerAbilitiesFlags>,
}
/// Player Action
#[deprecated]
pub struct PlayerAction;
mod player_command;
pub use player_command::*;
mod player_input;
pub use player_input::*;
/// Response to a Ping Packet
pub struct Pong {
    /// Response id
    pub id: Int,
}
/// Change Recipe Book Settings
#[deprecated]
pub struct ChangeRecipeBookSettings;
///
#[deprecated]
pub struct SetSeenRecipe;
///
#[deprecated]
pub struct RenameItem;
mod resource_pack_response;
pub use resource_pack_response::*;
///
#[deprecated]
pub struct SeenAdvancements;
///
#[deprecated]
pub struct SelectTrade;
///
#[deprecated]
pub struct SetBeaconEffect;
///
#[deprecated]
pub struct SetHeldItem;
///
#[deprecated]
pub struct ProgramCommandBlock;
///
#[deprecated]
pub struct ProgramCommandBlockMinecart;
///
#[deprecated]
pub struct SetCreativeModeSlot;
///
#[deprecated]
pub struct ProgramJigsawBlock;
///
#[deprecated]
pub struct ProgramStructureBlock;
///
#[deprecated]
pub struct UpdateSign;
/// Teleports a player in spectator mode to an entity
pub struct TeleportToEntity {
    /// The target
    pub target: UUID,
}
///
#[deprecated]
pub struct UseItemOn;
///
#[deprecated]
pub struct UseItem;
