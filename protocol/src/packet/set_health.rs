use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent by the server. It sets the health of the player it is sent to. The SetHealth packet should
/// no longer be used. Instead, the health attribute should be used so that the health and maximum
/// health may be changed directly.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetHealth {
    /// The new health of the player.
    pub health: VarI32,
}
