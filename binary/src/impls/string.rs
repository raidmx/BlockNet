use bytes::{Buf, BufMut};
use crate::{generate, Decode, Encode, Prefix, Reader, Writer};

generate!(RefString, <P: Prefix>, &'a str, 'a);
generate!(CString, <P: Prefix>, String);

impl<P: Prefix> Encode for RefString<'_, P> {
    fn encode(&self, w: &mut Writer) {
        P::from(self.len()).encode(w);
        w.put_slice(self.val.as_ref());
    }
}

impl<'a, P: Prefix> Decode<'a> for RefString<'a, P> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let len = P::decode(r)?.into();
        if r.remaining() < len {
            return None;
        }

        match std::str::from_utf8(&r[0..len]) {
            Ok(val) => Some(RefString::new(val)),
            Err(_) => None
        }
    }
}

impl<P: Prefix> Encode for CString<P> {
    fn encode(&self, w: &mut Writer) {
        P::from(self.len()).encode(w);
        w.put_slice(self.val.as_ref());
    }
}

impl<P: Prefix> Decode<'_> for CString<P> {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let len = P::decode(r)?.into();
        if r.remaining() < len {
            return None;
        }

        let mut bytes = vec![0u8; len];
        r.copy_to_slice(&mut bytes);

        match String::from_utf8(bytes) {
            Ok(val) => Some(CString::new(val)),
            Err(_) => None
        }
    }
}