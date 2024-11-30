use bytes::{Buf, BufMut};
use binary::{Decode, Encode, Reader, Writer};

#[derive(Debug)]
pub struct Magic;

impl Encode for Magic {
    fn encode(&self, w: &mut Writer) {
        w.put_slice(&[0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78]);
    }
}

impl Decode<'_> for Magic {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        if r.remaining() < 16 {
            return None;
        }
        
        r.advance(16);
        Some(Magic)
    }
}