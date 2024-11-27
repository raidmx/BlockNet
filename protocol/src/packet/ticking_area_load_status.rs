use derive::{Decode, Encode, Packet};

/// Sent by the server to the client to notify the client of a ticking area's loading status.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct TickingAreasLoadStatus {
    /// True if the server is waiting for the area's preload.
    pub preload: bool,
}
