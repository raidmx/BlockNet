use bytes::Bytes;
use binary::v64;
use derive::{Decode, Encode, Packet};

/// Sent by the server to the client. It does not seem to do anything when sent to the normal client
/// in 1.16.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct DebugInfo {
    /// The unique ID of the player that the packet is sent to.
    pub player_unique_id: v64,
    /// The debug data.
    pub data: Bytes,
}
