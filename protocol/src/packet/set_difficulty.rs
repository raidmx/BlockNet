use derive::{Decode, Encode, Packet};

use crate::types::world::Difficulty;

/// Sent by the server to update the client-side difficulty of the client. The actual effect of this
/// packet on the client isn't very significant, as the difficulty is handled server-side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetDifficulty {
    /// The new difficulty that the world has.
    pub difficulty: Difficulty,
}
