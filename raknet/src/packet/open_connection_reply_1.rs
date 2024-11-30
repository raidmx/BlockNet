use binary::{b16, b64};
use derive::{Decode, Encode, Packet};
use crate::types::Magic;

#[derive(Debug, Encode, Decode, Packet)]
pub struct OpenConnectionReply1 {
    pub magic: Magic,
    pub guid: b64,
    pub secure: bool,
    pub mtu: b16
}