use bytes::Buf;
use binary::{Decode, Encode, Reader, Writer};
use crate::types::Magic;

#[derive(Debug)]
pub struct OpenConnectionRequest1 {
    pub magic: Magic,
    pub protocol: u8,
    pub max_size: usize
}

impl Encode for OpenConnectionRequest1 {
    fn encode(&self, w: &mut Writer) {
        self.magic.encode(w);
        self.protocol.encode(w);
        w.advance(self.max_size - w.len() - 28)
    }
}

impl Decode<'_> for OpenConnectionRequest1 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let max_size = r.len() + 1 + 28;
        r.advance(16);
        
        Some(Self{
            magic: Magic,
            protocol: u8::decode(r)?,
            max_size
        })
    }
}