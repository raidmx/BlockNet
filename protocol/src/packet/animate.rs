use binary::w64;
use derive::{Decode, Encode, Packet};

#[repr(u32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
/// Each variant contains the runtime ID of the player that the animation should be played upon. The
/// runtime ID is unique for each world session, and entities are generally identified in packets
/// using this runtime ID.
pub enum AnimateAction {
    SwingArm(w64) = 1,
    StopSleep(w64) = 3,
    CriticalHit(w64) = 4,
    MagicCriticalHit(w64) = 5,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowRight(w64, f32) = 128,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowLeft(w64, f32) = 129,
}

/// Sent by the server to send a player animation from one player to all viewers of that player. It
/// is used for a couple of actions, such as arm swimming and critical hits.\
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct Animate {
    /// The action type to execute.
    pub action_type: AnimateAction,
}
