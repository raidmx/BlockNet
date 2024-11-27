use derive::{Decode, Encode, Packet};

/// Sent by the server to show the XBOX Live profile of one player to another.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ShowProfile {
    /// The XBOX Live User ID of the player whose profile should be shown to the player. If it is
    /// not a valid XUID, the client ignores the packet.
    pub xuid: String,
}
