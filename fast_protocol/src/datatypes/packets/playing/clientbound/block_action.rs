use binary_utils::DataWriter;
use datatypes::{ImportantFunctions as _, Position, VarInt};

/// All datatypes used in `BlockActions` are defined here, that are needed for action and parameter
/// data
///
/// # Source
/// <https://wiki.vg/Block_Actions>
pub mod block_actions {
    #[allow(missing_docs)]
    #[derive(Clone, Copy)]
    pub enum PistionActions {
        Extend = 0,
        Retract = 1,
        /// This will result in a sticky piston not pulling back the attached block
        CancleOngoingExtension = 2,
    }
    #[allow(missing_docs)]
    #[derive(Clone, Copy)]
    pub enum PistonParameters {
        Down = 0,
        Up = 1,
        South = 2,
        West = 3,
        North = 4,
        East = 5,
    }
    #[allow(missing_docs)]
    #[derive(Clone, Copy)]
    pub enum BellParameters {
        Down = 0,
        Up = 1,
        South = 2,
        West = 3,
        North = 4,
        East = 5,
    }
}

/// This enum represents a block action
#[allow(missing_docs)]
pub enum BlockActionData {
    NoteBlock,
    Piston {
        action: block_actions::PistionActions,
        parameter: block_actions::PistonParameters,
    },
    Chest {
        amount: i32,
    },
    Spawner,
    EndGateway,
    ShulkerBox {
        amount: i32,
    },
    Bell {
        /// The direction that the bell is rang from, if from a player, then it uses the location
        /// of the player, otherwise it uses the orientation of the bell
        direction: block_actions::BellParameters,
    },
    Nothing,
}
impl DataWriter for BlockActionData {
    async fn write(&self, writer: &mut (impl tokio::io::AsyncWrite + Unpin)) -> binary_utils::Result<()> {
        match self {
            Self::Nothing | Self::NoteBlock => {
                VarInt::new(0).write(writer).await?;
                VarInt::new(0).write(writer).await?;
            }
            Self::Piston { action, parameter } => {
                VarInt::new(*action as i32).write(writer).await?;
                VarInt::new(*parameter as i32).write(writer).await?;
            }
            Self::Chest { amount } => {
                VarInt::new(1).write(writer).await?;
                VarInt::new(*amount).write(writer).await?;
            }
            Self::Spawner => {
                VarInt::new(1).write(writer).await?;
                VarInt::new(0).write(writer).await?;
            }
            Self::EndGateway => {
                VarInt::new(1).write(writer).await?;
                VarInt::new(0).write(writer).await?;
            }
            Self::ShulkerBox { amount } => {
                VarInt::new(1).write(writer).await?;
                VarInt::new(*amount).write(writer).await?;
            }
            Self::Bell { direction } => {
                VarInt::new(1).write(writer).await?;
                VarInt::new(*direction as i32).write(writer).await?;
            }
        }
        Ok(())
    }
}
/// This packet is used to start and stop block action animations
#[allow(missing_docs)]
pub struct BlockAction {
    pub location: Position,
    pub action: BlockActionData,
    pub block_type: VarInt,
}
