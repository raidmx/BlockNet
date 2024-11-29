use num_derive::{FromPrimitive, ToPrimitive};
use binary::v32;
use derive::{Decode, Encode};
use crate::types::BlockPos;

#[repr(i32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum PlayerActionType {
    StartBreak(BlockDetail),
    AbortBreak(BlockDetail),
    StopBreak,
    GetUpdatedBlock,
    DropItem,
    StartSleeping,
    StopSleeping,
    Respawn,
    Jump,
    StartSprint,
    StopSprint,
    StartSneak,
    StopSneak,
    CreativePlayerDestroyBlock,
    DimensionChangeDone,
    StartGlide,
    StopGlide,
    BuildDenied,
    CrackBreak(BlockDetail),
    ChangeSkin,
    SetEnchantmentSeed,
    StartSwimming,
    StopSwimming,
    StartSpinAttack,
    StopSpinAttack,
    StartBuildingBlock,
    PredictDestroyBlock(BlockDetail),
    ContinueDestroyBlock(BlockDetail),
    StartItemUseOn,
    StopItemUseOn,
    HandledTeleport,
    MissedSwing,
    StartCrawling,
    StopCrawling,
    StartFlying,
    StopFlying,
    ClientAckServerData,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockDetail {
    pub pos: BlockPos,
    pub face: v32
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MoveMode {
    Normal,
    Reset,
    Teleport,
    Rotation,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
#[encoding(type = w32)]
pub enum InputMode {
    #[default]
    None,
    Mouse,
    Touch,
    GamePad,
    MotionController,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
#[encoding(type = v32)]
pub enum InteractionModel {
    #[default]
    Touch,
    Crosshair,
    Classic,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlayerMovementSettings {
    pub movement_type: v32,
    pub rewind_history_size: v32,
    pub server_authoritative_block_breaking: bool,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlayerBlockAction {
    pub action: PlayerActionType,
    pub block_pos: BlockPos,
    pub face: i32,
}