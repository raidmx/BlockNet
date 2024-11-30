use bytes::BufMut;
use binary::{Decode, Encode, Reader, Writer};
use derive::Packet;

#[derive(Debug, Packet)]
pub struct Game<'a> {
    pub data: &'a [u8]
}

impl<'a> Encode for Game<'a> {
    fn encode(&self, w: &mut Writer) {
        w.put_slice(self.data);
    }
}

impl<'a> Decode<'a> for Game<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        Some(Self {data: &r[..]})
    }
}