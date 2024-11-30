use std::net::{IpAddr, SocketAddr};
use bytes::{Buf, BufMut};
use uuid::Uuid;
use crate::{b16, b32, Decode, Encode, Reader, Writer};

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

impl Encode for Uuid {
    fn encode(&self, w: &mut Writer) {
        w.put_slice(self.to_bytes_le().as_slice());
    }
}

impl Decode<'_> for Uuid {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        if r.remaining() < 16 {
            return None;
        }

        let slice = &r[..16];
        r.advance(16);

        match Uuid::from_slice_le(&slice) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
}

impl Encode for SocketAddr {
    fn encode(&self, w: &mut Writer) {
        match self.ip() {
            IpAddr::V4(addr) => {
                4_u8.encode(w);
                addr.octets().encode(w);
                b16::new(self.port() as i16).encode(w);
            },
            IpAddr::V6(addr) => {
                6_u8.encode(w);
                b16::new(23).encode(w);
                b16::new(self.port() as i16).encode(w);

                b32::new(0).encode(w);
                addr.octets().encode(w);
                b32::new(0).encode(w);
            }
        }
    }
}

impl Decode<'_> for SocketAddr {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        match u8::decode(r)? {
            4 => {
                let octets = <[u8; 4]>::decode(r)?;
                let port = b16::decode(r)?.value() as u16;

                let addr = SocketAddr::new(IpAddr::V4(octets.into()), port);
                Some(addr)
            },
            6 => {
                r.advance(2);
                let port = b16::decode(r)?.value() as u16;

                r.advance(4);
                let octets = <[u8; 16]>::decode(r)?;
                r.advance(4);

                let addr = SocketAddr::new(IpAddr::V6(octets.into()), port);
                Some(addr)
            }
            _ => None
        }
    }
}