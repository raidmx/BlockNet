pub mod numerics;
pub mod prefixed;
pub mod enums;

pub use numerics::*;
pub use enums::*;

use bytes::{Bytes, BytesMut};

pub trait Binary: Default {
    fn serialize(&self, writer: &mut BytesMut);
    fn deserialize(reader: &mut Bytes) -> Self;
}