use bytes::{Buf, BufMut};
use crate::{generate, impl_numeric, Decode, Encode, Prefix, Reader, Variant, Writer};

generate!(VarI32, <>, i32);
generate!(VarU32, <>, u32);
generate!(VarI64, <>, i64);
generate!(VarU64, <>, u64);

impl Encode for VarI32 {
    fn encode(&self, w: &mut Writer) {
        let mut u = (**self as u32) << 1;
        if **self < 0 {
            u = !u;
        }
        while u >= 0x80 {
            w.put_u8(u as u8 | 0x80);
            u >>= 7;
        }
        w.put_u8(u as u8);
    }
}

impl Decode<'_> for VarI32 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            if r.remaining() < 1  {
                return None;
            }

            let b = r.get_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return if v & 1 != 0 { Some(Self::new(!x)) } else { Some(Self::new(x)) };
            }
        }
        None
    }
}

impl Encode for VarU32 {
    fn encode(&self, w: &mut Writer) {
        let mut x = **self;
        while x >= 0x80 {
            w.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        w.put_u8(x as u8);
    }
}

impl Decode<'_> for VarU32 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            if r.remaining() < 1  {
                return None;
            }

            let b = r.get_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                return Some(Self::new(v))
            }
        }
        None
    }
}

impl Encode for VarI64 {
    fn encode(&self, w: &mut Writer) {
        let mut u = (**self as u64) << 1;
        if **self < 0 {
            u = !u;
        }
        while u >= 0x80 {
            w.put_u8(u as u8 | 0x80);
            u >>= 7;
        }
        w.put_u8(u as u8);
    }
}

impl Decode<'_> for VarI64 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            if r.remaining() < 1  {
                return None;
            }

            let b = r.get_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return if v & 1 != 0 { Some(Self::new(!x)) } else { Some(Self::new(x)) };
            }
        }
        None
    }
}

impl Encode for VarU64 {
    fn encode(&self, w: &mut Writer) {
        let mut x = **self;
        while x >= 0x80 {
            w.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        w.put_u8(x as u8);
    }
}

impl Decode<'_> for VarU64 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            if r.remaining() < 1  {
                return None;
            }

            let b = r.get_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                return Some(Self::new(v))
            }
        }
        None
    }
}

impl_numeric!(VarI32, <>, i32);
impl_numeric!(VarU32, <>, u32);
impl_numeric!(VarI64, <>, i64);
impl_numeric!(VarU64, <>, u64);