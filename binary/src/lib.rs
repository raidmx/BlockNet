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

pub trait Encode : Debug {
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

pub trait Decode<'a> : Sized + Debug {
    /// Reads this object from the provided byte slice.
    ///
    /// Implementations of `Decode` are expected to shrink the slice from the
    /// front as bytes are read.
    fn decode(r: &mut Reader<'a>) -> Option<Self>;
}

/// EnumEncoder is a trait implemented by Enums to serialize and deserialize enumerations
pub trait EnumEncoder<V: Variant> : Debug + Sized {
    fn encode(&self, w: &mut Writer);
    fn decode(r: &mut Reader) -> Option<Self>;
}

/// Prefix is a trait implemented by all those numeric types that implement the
/// [`Encode`] and [`Decode`] trait and as well as can be converted to and from
/// usize.
pub trait Prefix: Encode + for <'a> Decode<'a> + From<usize> + Into<usize> {}

/// Variant is a trait implemented by all those numeric types that implement the
/// [`Encode`] and [`Decode`] trait and as well as can be converted to and from
/// isize.
pub trait Variant: Encode + for <'a> Decode<'a> + From<isize> + Into<isize> {}