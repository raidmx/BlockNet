use binary::b64;
use derive::{Decode, Encode, Packet};
use crate::types::Magic;

#[derive(Debug, Encode, Decode, Packet)]
pub struct IncompatibleProtocol {
    pub protocol: u8,
    pub magic: Magic,
    pub guid: b64
}