use bytes::{Buf, BufMut};
use crate::{generate, Decode, Encode, Numeric, Prefix, Reader, VarU32, Writer};

generate!(RefString, <P: Prefix>, &'a str, 'a);
generate!(CString, <P: Prefix>, String);

impl<P: Prefix> Encode for RefString<'_, P> {
    fn encode(&self, w: &mut Writer) {
        P::from_usize(self.len()).encode(w);
        w.put_slice(self.val.as_ref());
    }
}

impl<'a, P: Prefix> Decode<'a> for RefString<'a, P> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let len = P::decode(r)?.to_usize();
        if r.remaining() < len {
            return None;
        }

        match std::str::from_utf8(&r[0..len]) {
            Ok(val) => {
                r.advance(len);
                Some(RefString::new(val))
            },
            Err(_) => None
        }
    }
}

impl<P: Prefix> Encode for CString<P> {
    fn encode(&self, w: &mut Writer) {
        P::from_usize(self.len()).encode(w);
        w.put_slice(self.val.as_ref());
    }
}

impl<P: Prefix> Decode<'_> for CString<P> {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let len = P::decode(r)?.to_usize();
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

impl Encode for str {
    fn encode(&self, w: &mut Writer) {
        VarU32::from_usize(self.len()).encode(w);
        w.put_slice(self.as_ref());
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let len = VarU32::decode(r)?.to_usize();
        if r.remaining() < len {
            return None;
        }
        
        match std::str::from_utf8(&r[..len]) {
            Ok(v) => {
                r.advance(len);
                Some(v)
            },
            Err(_) => None
        }
    }
}

impl Encode for String {
    fn encode(&self, w: &mut Writer) {
        self.as_str().encode(w);
    }
}

impl Decode<'_> for String {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(<&str>::decode(r)?.into())
    }
}