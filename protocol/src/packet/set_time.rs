use binary::v32;
use derive::{Decode, Encode, Packet};

/// Sent by the server to update the current time client-side. The client actually advances time
/// client-side by itself, so this packet does not need to be sent each tick. It is a means of
/// synchronising time between server and client.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetTime {
    /// The current time. The time is not limited to 24000 (time of day), but continues progressing
    /// after that.
    pub time: v32,
}
