use bytes::{Bytes, BytesMut};
use crate::Numeric;

pub trait EnumEncoder<N: Numeric> : Sized {
    fn encode(&self, writer: &mut BytesMut);
    fn decode(reader: &mut Bytes) -> Option<Self>;
}