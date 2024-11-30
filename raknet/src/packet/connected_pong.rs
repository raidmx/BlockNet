use binary::b64;
use derive::{Decode, Encode, Packet};

#[derive(Debug, Encode, Decode, Packet)]
pub struct ConnectedPong {
    pub ping_time: b64,
    pub pong_time: b64
}