use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent by the client to the server to initiate a cooldown on an item. The purpose of this packet
/// isn't entirely clear.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ClientStartItemCooldown {
    /// The category of the item to start the cooldown on.
    pub category: String,
    /// The duration of ticks the cooldown should last.
    pub duration: VarI32,
}
