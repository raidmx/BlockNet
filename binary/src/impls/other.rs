use bytes::{Buf, BufMut};
use crate::{Decode, Encode, Reader, Writer};

impl Encode for bool {
    #[inline]
    fn encode(&self, w: &mut Writer) {
        w.put_u8(u8::from(*self));
    }
}

impl Decode<'_> for bool {
    #[inline]
    fn decode(r: &mut Reader) -> Option<Self> {
        if r.remaining() < 1 {
            return None;
        }

        let n = r.get_u8();
        Some(n == 1)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, mut w: &mut Writer) {
        match self {
            Some(t) => {
                true.encode(&mut w);
                t.encode(w)
            }
            None => false.encode(w),
        }
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        Some(if bool::decode(r)? { Some(T::decode(r)?) } else { return None })
    }
}