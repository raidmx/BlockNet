use binary::b64;
use derive::{Decode, Encode, Packet};

#[derive(Debug, Encode, Decode, Packet)]
pub struct ConnectedPing {
    pub ping_time: b64
}