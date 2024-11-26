pub mod impls;
pub mod order;

pub use impls::*;
pub use order::*;

use std::fmt::Debug;
use bytes::BytesMut;

/// Writer is a type alias for a BytesMut instance.
pub type Writer = BytesMut;

/// Reader is a type alias for a shared reference to a byte slice. This
/// byte slice is usually obtained from a [`Bytes`] or a [`BytesMut`]
/// instance.
pub type Reader<'a> = &'a [u8];

pub trait Encode: Debug {
    /// Writes this object to the provided writer.
    ///
    /// If this type also implements [`Decode`] then successful calls to this
    /// function returning `Ok(())` must always successfully [`decode`] using
    /// the data that was written to the writer. The exact number of bytes
    /// that were originally written must be consumed during the decoding.
    ///
    /// [`decode`]: Decode::decode
    fn encode(&self, w: &mut Writer);
}

pub trait Decode<'a> : Debug + Sized {
    /// Reads this object from the provided byte slice.
    ///
    /// Implementations of `Decode` are expected to shrink the slice from the
    /// front as bytes are read.
    fn decode(r: &mut Reader<'a>) -> Option<Self>;
}

/// EnumEncoder is a trait implemented by Enums to serialize and deserialize enum variants.
/// It uses the Variant trait to specify what type of integer to use for serializing data.
pub trait EnumEncoder: Debug {
    fn write<V: Variant>(&self, w: &mut Writer);
}

/// EnumDecoder is a trait implemented by Enums to serialize and deserialize enum variants.
/// It uses the Variant trait to specify what type of integer to use for serializing data.
pub trait EnumDecoder : Debug + Sized {
    fn read<V: Variant>(r: &mut Reader) -> Option<Self>;
}

/// Numeric trait is implemented for all those integer types that can be converted
/// from usize & isize and vice versa
pub trait Numeric: Sized {
    fn from_usize(val: usize) -> Self;
    fn to_usize(self) -> usize;

    fn from_isize(val: isize) -> Self;
    fn to_isize(self) -> isize;
}

/// Prefix is a trait implemented by all those numeric types that implement the
/// [`Encode`] and [`Decode`] trait and as well as can be converted to and from
/// usize.
pub trait Prefix: Encode + for <'a> Decode<'a> + Numeric {}

/// Variant is a trait implemented by all those numeric types that implement the
/// [`Encode`] and [`Decode`] trait and as well as can be converted to and from
/// isize.
pub trait Variant: Encode + for <'a> Decode<'a> + Numeric {}