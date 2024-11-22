#![allow(non_camel_case_types)]

use crate::Binary;
use bytes::{Buf, BufMut, BytesMut, Bytes};

pub trait Numeric {
    fn write_usize(writer: &mut BytesMut, value: usize);
    fn read_usize(reader: &mut Bytes) -> usize;
    fn write_isize(writer: &mut BytesMut, value: isize);
    fn read_isize(reader: &mut Bytes) -> isize;
    fn from_usize(value: usize) -> Self;
    fn to_usize(self) -> usize;
}

macro_rules! impl_generic {
    ($type:ident, $read:ident, $write:ident) => {
        impl crate::Binary for $type {
            #[inline]
            fn serialize(&self, writer: &mut bytes::BytesMut) {
                writer.$write(*self);
            }

            #[inline]
            fn deserialize(reader: &mut bytes::Bytes) -> Self {
                reader.$read()
            }
        }

        impl Numeric for $type {
            #[inline]
            fn write_usize(writer: &mut bytes::BytesMut, value: usize) {
                (value as $type).serialize(writer);
            }

            #[inline]
            fn read_usize(reader: &mut bytes::Bytes) -> usize {
                $type::deserialize(reader) as usize
            }

            #[inline]
            fn write_isize(writer: &mut bytes::BytesMut, value: isize) {
                (value as $type).serialize(writer);
            }

            #[inline]
            fn read_isize(reader: &mut bytes::Bytes) -> isize {
                $type::deserialize(reader) as isize
            }

            #[inline]
            fn from_usize(value: usize) -> $type {
                value as $type
            }

            #[inline]
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    };
}

impl_generic!(u8, get_u8, put_u8);
impl_generic!(i8, get_i8, put_i8);
impl_generic!(u16, get_u16_le, put_u16_le);
impl_generic!(i16, get_i16_le, put_i16_le);
impl_generic!(u32, get_u32_le, put_u32_le);
impl_generic!(i32, get_i32_le, put_i32_le);
impl_generic!(u64, get_u64_le, put_u64_le);
impl_generic!(i64, get_i64_le, put_i64_le);
impl_generic!(f32, get_f32_le, put_f32_le);
impl_generic!(f64, get_f64_le, put_f64_le);

macro_rules! create_derived {
    ($new_type:ident, $base_type:ty) => {
        #[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
        pub struct $new_type(pub $base_type);

        impl std::ops::Deref for $new_type {
            type Target = $base_type;
            fn deref(&self) -> &$base_type {
                &self.0
            }
        }

        impl std::ops::DerefMut for $new_type {
            fn deref_mut(&mut self) -> &mut $base_type {
                &mut self.0
            }
        }

        impl From<$base_type> for $new_type {
            #[inline]
            fn from(value: $base_type) -> Self {
                $new_type(value)
            }
        }

        impl From<$new_type> for $base_type {
            #[inline]
            fn from(value: $new_type) -> $base_type {
                value.0
            }
        }

        impl Numeric for $new_type {
            #[inline]
            fn write_usize(writer: &mut bytes::BytesMut, value: usize) {
                Self(value as $base_type).serialize(writer);
            }

            #[inline]
            fn read_usize(reader: &mut bytes::Bytes) -> usize {
                $new_type::deserialize(reader).0 as usize
            }

            #[inline]
            fn write_isize(writer: &mut bytes::BytesMut, value: isize) {
                Self(value as $base_type).serialize(writer);
            }

            #[inline]
            fn read_isize(reader: &mut bytes::Bytes) -> isize {
                $new_type::deserialize(reader).0 as isize
            }

            #[inline]
            fn from_usize(value: usize) -> $new_type {
                Self(value as $base_type)
            }

            #[inline]
            fn to_usize(self) -> usize {
                self.0 as usize
            }
        }
    };
}

create_derived!(b16, i16);
create_derived!(n16, u16);
create_derived!(u24, u32);
create_derived!(b32, i32);
create_derived!(n32, u32);
create_derived!(v32, i32);
create_derived!(w32, u32);
create_derived!(b64, i64);
create_derived!(n64, u64);
create_derived!(v64, i64);
create_derived!(w64, u64);
create_derived!(d32, f32);
create_derived!(d64, f64);

macro_rules! impl_derived {
    ($type:ident, $read:ident, $write:ident) => {
        impl crate::Binary for $type {
            #[inline]
            fn serialize(&self, writer: &mut bytes::BytesMut) {
                writer.$write(**self);
            }

            #[inline]
            fn deserialize(reader: &mut bytes::Bytes) -> Self {
                Self(reader.$read())
            }
        }
    };
}

impl_derived!(b16, get_i16, put_i16);
impl_derived!(n16, get_u16, put_u16);
impl_derived!(b32, get_i32, put_i32);
impl_derived!(n32, get_u32, put_u32);
impl_derived!(b64, get_i64, put_i64);
impl_derived!(n64, get_u64, put_u64);
impl_derived!(d32, get_f32, put_f32);
impl_derived!(d64, get_f64, put_f64);

impl Binary for u24 {
    #[inline]
    fn serialize(&self, writer: &mut BytesMut) {
        writer.put_slice(&[**self as u8, (**self >> 8) as u8, (**self >> 16) as u8]);
    }

    #[inline]
    fn deserialize(reader: &mut Bytes) -> Self {
        let slice = &reader.chunk()[..3];
        let value = slice[0] as u32 | (slice[1] as u32) << 8 | (slice[2] as u32) << 16;

        reader.advance(3);
        Self(value)
    }
}

impl Binary for v32 {
    fn serialize(&self, writer: &mut BytesMut) {
        let mut u = (self.0 as u32) << 1;
        if self.0 < 0 {
            u = !u;
        }
        while u >= 0x80 {
            writer.put_u8(u as u8 | 0x80);
            u >>= 7;
        }
        writer.put_u8(u as u8);
    }

    fn deserialize(reader: &mut Bytes) -> Self {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = reader.get_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return if v & 1 != 0 { Self(!x) } else { Self(x) };
            }
        }
        panic!("varint i32 overflow");
    }
}

impl Binary for w32 {
    fn serialize(&self, writer: &mut BytesMut) {
        let mut x = **self;
        while x >= 0x80 {
            writer.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        writer.put_u8(x as u8);
    }

    fn deserialize(reader: &mut Bytes) -> Self {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = reader.get_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                return Self(v)
            }
        }
        panic!("varint u32 overflow");
    }
}

impl Binary for v64 {
    fn serialize(&self, writer: &mut BytesMut) {
        let mut u = (**self as u64) << 1;
        if **self < 0 {
            u = !u;
        }
        while u >= 0x80 {
            writer.put_u8(u as u8 | 0x80);
            u >>= 7;
        }
        writer.put_u8(u as u8);
    }

    fn deserialize(reader: &mut Bytes) -> Self {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = reader.get_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return if v & 1 != 0 { Self(!x) } else { Self(x) };
            }
        }
        panic!("varint i64 overflow");
    }
}
impl Binary for w64 {
    fn serialize(&self, writer: &mut BytesMut) {
        let mut x = **self;
        while x >= 0x80 {
            writer.put_u8(x as u8 | 0x80);
            x >>= 7;
        }
        writer.put_u8(x as u8);
    }

    fn deserialize(reader: &mut Bytes) -> Self {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = reader.get_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                return Self(v)
            }
        }
        panic!("varint u64 overflow");
    }
}