pub mod numerics;
pub mod sized;

pub use numerics::*;
pub use sized::*;

use bytes::{Bytes, BytesMut};

pub trait Binary: Default {
    fn encode(&self, writer: &mut BytesMut);
    fn decode(reader: &mut Bytes) -> Self;
}