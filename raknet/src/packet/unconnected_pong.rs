use binary::{b16, b64, RefString};
use derive::{Decode, Encode, Packet};
use crate::types::Magic;

#[derive(Debug, Encode, Decode, Packet)]
pub struct UnconnectedPong<'a> {
    pub pong_time: b64,
    pub guid: b64,
    pub magic: Magic,
    pub data: RefString<'a, b16>
}