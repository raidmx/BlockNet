#![allow(non_camel_case_types)]

use bytes::{Buf, BufMut};
use crate::{generate, impl_numeric_ordered, Decode, Encode, Reader, Variant, Writer, Prefix, Numeric};

generate!(v32, <>, i32);
generate!(w32, <>, u32);
generate!(v64, <>, i64);
generate!(w64, <>, u64);

impl Encode for v32 {
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

impl Decode<'_> for v32 {
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

impl Encode for w32 {
    fn encode(&self, w: &mut Writer) {
        let mut x = **self;
        while x >= 0x80 {
            w.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        w.put_u8(x as u8);
    }
}

impl Decode<'_> for w32 {
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

impl Encode for v64 {
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

impl Decode<'_> for v64 {
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

impl Encode for w64 {
    fn encode(&self, w: &mut Writer) {
        let mut x = **self;
        while x >= 0x80 {
            w.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        w.put_u8(x as u8);
    }
}

impl Decode<'_> for w64 {
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

impl_numeric_ordered!(v32, <>, i32);
impl_numeric_ordered!(w32, <>, u32);
impl_numeric_ordered!(v64, <>, i64);
impl_numeric_ordered!(w64, <>, u64);