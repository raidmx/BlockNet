use std::net::SocketAddr;
use binary::{b16, b64};
use derive::{Decode, Encode, Packet};
use crate::types::Magic;

#[derive(Debug, Encode, Decode, Packet)]
pub struct OpenConnectionReply2 {
    pub magic: Magic,
    pub guid: b64,
    pub addr: SocketAddr,
    pub mtu: b16,
    pub secure: bool
}