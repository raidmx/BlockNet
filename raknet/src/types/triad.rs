use bytes::BufMut;
use binary::{generate, Decode, Encode, Reader, Writer};

generate!(U24, <>, u32);

impl Encode for U24 {
    fn encode(&self, w: &mut Writer) {
        let a = **self as u8;
        let b = (**self >> 8) as u8;
        let c = (**self >> 16) as u8;

        w.put_slice(&[a, b, c]);
    }
}

impl Decode<'_> for U24 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let a = u8::decode(r)? as u32;
        let b = (u8::decode(r)? as u32) << 8;
        let c = (u8::decode(r)? as u32) << 16;

        Some(Self::new(a | b | c))
    }
}