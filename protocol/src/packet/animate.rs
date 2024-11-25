use binary::VarU64;
use derive::{Decode, Encode, Packet};

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = VarI32)]
/// Each variant contains the runtime ID of the player that the animation should be played upon. The
/// runtime ID is unique for each world session, and entities are generally identified in packets
/// using this runtime ID.
pub enum AnimateAction {
    SwingArm(VarU64) = 1,
    StopSleep(VarU64) = 3,
    CriticalHit(VarU64) = 4,
    MagicCriticalHit(VarU64) = 5,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowRight(VarU64, f32) = 128,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowLeft(VarU64, f32) = 129,
}

/// Sent by the server to send a player animation from one player to all viewers of that player. It
/// is used for a couple of actions, such as arm swimming and critical hits.\
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct Animate {
    /// The action type to execute.
    pub action_type: AnimateAction,
}