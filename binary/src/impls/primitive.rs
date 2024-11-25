#![allow(non_camel_case_types)]

use bytes::{Buf, BufMut};
use crate::{generate, ByteOrder, Decode, Encode, Prefix, Reader, Variant, Writer, BE};

generate!(U8, <>, u8);
generate!(I8, <>, i8);
generate!(U16, <E: ByteOrder>, u16);
generate!(I16, <E: ByteOrder>, i16);
generate!(U32, <E: ByteOrder>, u32);
generate!(I32, <E: ByteOrder>, i32);
generate!(U64, <E: ByteOrder>, u64);
generate!(I64, <E: ByteOrder>, i64);
generate!(F32, <E: ByteOrder>, f32);
generate!(F64, <E: ByteOrder>, f64);

pub type b16 = I16<BE>;
pub type n16 = U16<BE>;
pub type b32 = I32<BE>;
pub type n32 = U32<BE>;
pub type b64 = I64<BE>;
pub type n64 = U64<BE>;
pub type d32 = F32<BE>;
pub type d64 = F64<BE>;

impl Encode for U8 {
    fn encode(&self, w: &mut Writer) {
        w.put_u8(self.val);
    }
}

impl Decode<'_> for U8 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        if r.remaining() < 1 {
            return None;
        }

        Some(r.get_u8().into())
    }
}

impl Encode for I8 {
    fn encode(&self, w: &mut Writer) {
        w.put_i8(self.val);
    }
}

impl Decode<'_> for I8 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        if r.remaining() < 1 {
            return None;
        }

        Some(r.get_i8().into())
    }
}

macro_rules! impl_generic {
    ($type:ident, $read:ident, $write:ident) => {
        impl Encode for $type {
            #[inline]
            fn encode(&self, w: &mut Writer) {
                w.$write(*self);
            }
        }

        impl Decode<'_> for $type {
            #[inline]
            fn decode(r: &mut Reader) -> Option<Self> {
                if r.remaining() < std::mem::size_of::<$type>() {
                    return None;
                }

                Some(r.$read())
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

macro_rules! impl_ordered {
    ($type:ident, <$($gen:ident: $gen_constraint:ident),*>, $read:ident, $write:ident) => {
        impl<$($gen: $gen_constraint),*> Encode for $type<$($gen),*> {
            #[inline]
            fn encode(&self, w: &mut Writer) {
                E::$write(w, self.val);
            }
        }

        impl<$($gen: $gen_constraint),*> Decode<'_> for $type<$($gen),*> {
            #[inline]
            fn decode(r: &mut Reader) -> Option<Self> {
                Some(Self::new(E::$read(r)?))
            }
        }
    };
}

impl_ordered!(U16, <E: ByteOrder>, get_u16, put_u16);
impl_ordered!(I16, <E: ByteOrder>, get_i16, put_i16);
impl_ordered!(U32, <E: ByteOrder>, get_u32, put_u32);
impl_ordered!(I32, <E: ByteOrder>, get_i32, put_i32);
impl_ordered!(U64, <E: ByteOrder>, get_u64, put_u64);
impl_ordered!(I64, <E: ByteOrder>, get_i64, put_i64);
impl_ordered!(F32, <E: ByteOrder>, get_f32, put_f32);
impl_ordered!(F64, <E: ByteOrder>, get_f64, put_f64);

#[macro_export]
macro_rules! impl_numeric {
    ($type:ident, <$($gen:ident: $gen_constraint:ident),*>, $base_type:ty) => {
        impl<$($gen: $gen_constraint),*> From<usize> for $type<$($gen),*> {
            fn from(value: usize) -> Self {
                Self::new(value as $base_type)
            }
        }

        impl<$($gen: $gen_constraint),*> From<$type<$($gen),*>> for usize {
            fn from(value: $type<$($gen),*>) -> usize {
                value.get() as usize
            }
        }

        impl<$($gen: $gen_constraint),*> From<isize> for $type<$($gen),*> {
            fn from(value: isize) -> Self {
                Self::new(value as $base_type)
            }
        }

        impl<$($gen: $gen_constraint),*> From<$type<$($gen),*>> for isize {
            fn from(value: $type<$($gen),*>) -> isize {
                value.get() as isize
            }
        }

        impl<$($gen: $gen_constraint),*> Prefix for $type<$($gen),*> {}
        impl<$($gen: $gen_constraint),*> Variant for $type<$($gen),*> {}
    };
}

impl_numeric!(U8, <>, u8);
impl_numeric!(I8, <>, i8);
impl_numeric!(U16, <E: ByteOrder>, u16);
impl_numeric!(I16, <E: ByteOrder>, i16);
impl_numeric!(U32, <E: ByteOrder>, u32);
impl_numeric!(I32, <E: ByteOrder>, i32);
impl_numeric!(U64, <E: ByteOrder>, u64);
impl_numeric!(I64, <E: ByteOrder>, i64);