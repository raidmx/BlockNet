use bytes::Buf;
use binary::{Decode, Encode, Reader, Writer};
use derive::Packet;
use crate::types::Magic;

#[derive(Debug, Packet)]
pub struct OpenConnectionRequest1 {
    pub magic: Magic,
    pub protocol: u8,
    pub max_size: usize
}

impl Encode for OpenConnectionRequest1 {
    fn encode(&self, w: &mut Writer) {
        self.magic.encode(w);
        self.protocol.encode(w);
        w.advance(self.max_size - w.len() - 28) // IP Header: 20 bytes, UDP Header: 8 bytes
    }
}

impl Decode<'_> for OpenConnectionRequest1 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let max_size = r.len() + 20 + 8 + 1; // IP Header: 20 bytes, UDP Header: 8 bytes, Packet ID: 1 byte
        r.advance(16);
        
        Some(Self{
            magic: Magic,
            protocol: u8::decode(r)?,
            max_size
        })
    }
}